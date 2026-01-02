<script lang="ts">
  import { onMount } from 'svelte';
  import { getAllInvoices, createInvoice, updateInvoiceStatus, getUninvoicedSessions, deleteInvoice } from '../api';
  import type { Invoice, NewInvoice, WorkSession } from '../types';
  import { FileText, Plus, DollarSign, Calendar, User, Send, CheckCircle, Clock, Trash2, X, Download, Eye, Printer, Mail, Edit, FileDown, Share2 } from 'lucide-svelte';
  import jsPDF from 'jspdf';
  import autoTable from 'jspdf-autotable';

  let invoices: Invoice[] = [];
  let uninvoicedSessions: WorkSession[] = [];
  let loading = true;
  let showCreateDialog = false;
  let showEditDialog = false;
  let selectedInvoice: Invoice | null = null;
  let editingInvoice: Invoice | null = null;
  let statusMessage = '';
  let statusType: 'success' | 'error' | '' = '';

  // Form state for create
  let clientName = '';
  let clientEmail = '';
  let selectedSessionIds: number[] = [];
  let notes = '';
  let dueDate = '';

  // Form state for edit
  let editClientName = '';
  let editClientEmail = '';
  let editNotes = '';
  let editDueDate = '';

  onMount(() => loadData());

  async function loadData() {
    loading = true;
    try {
      [invoices, uninvoicedSessions] = await Promise.all([
        getAllInvoices(),
        getUninvoicedSessions()
      ]);
    } catch (e) {
      console.error('Failed to load invoices:', e);
      showStatus('Failed to load invoices', 'error');
    } finally {
      loading = false;
    }
  }

  function showStatus(message: string, type: 'success' | 'error') {
    statusMessage = message;
    statusType = type;
    setTimeout(() => {
      statusMessage = '';
      statusType = '';
    }, 3000);
  }

  function openCreateDialog() {
    clientName = '';
    clientEmail = '';
    selectedSessionIds = [];
    notes = '';
    const due = new Date();
    due.setDate(due.getDate() + 30);
    dueDate = due.toISOString().split('T')[0];
    showCreateDialog = true;
  }

  function openEditDialog(invoice: Invoice) {
    editingInvoice = invoice;
    editClientName = invoice.client_name;
    editClientEmail = invoice.client_email || '';
    editNotes = invoice.notes || '';
    editDueDate = invoice.due_date || '';
    showEditDialog = true;
  }

  async function handleCreate() {
    if (!clientName || selectedSessionIds.length === 0) return;

    try {
      const finalDueDate = dueDate || new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString().split('T')[0];
      
      const newInvoice: NewInvoice = {
        client_name: clientName,
        client_email: clientEmail || undefined,
        session_ids: selectedSessionIds,
        notes: notes || undefined,
        due_date: finalDueDate,
      };
      await createInvoice(newInvoice);
      showCreateDialog = false;
      showStatus('Invoice created successfully!', 'success');
      await loadData();
    } catch (e) {
      console.error('Failed to create invoice:', e);
      showStatus('Failed to create invoice', 'error');
    }
  }

  async function handleStatusChange(invoice: Invoice, status: string) {
    try {
      // Capitalize the status for Rust enum parsing
      const capitalizedStatus = status.charAt(0).toUpperCase() + status.slice(1).toLowerCase();
      console.log('Updating invoice', invoice.id, 'to status:', capitalizedStatus);
      await updateInvoiceStatus(invoice.id, capitalizedStatus);
      showStatus(`Invoice marked as ${capitalizedStatus}!`, 'success');
      showEditDialog = false;
      await loadData();
    } catch (e) {
      console.error('Failed to update status:', e);
      showStatus('Failed to update status: ' + e, 'error');
    }
  }

  async function handleDelete(invoice: Invoice) {
    if (!confirm(`Delete invoice #${invoice.invoice_number}?`)) return;
    try {
      await deleteInvoice(invoice.id);
      showStatus('Invoice deleted', 'success');
      await loadData();
    } catch (e) {
      console.error('Failed to delete invoice:', e);
      showStatus('Failed to delete invoice', 'error');
    }
  }

  // ========== PDF GENERATION ==========
  function generatePDF(invoice: Invoice): jsPDF {
    const doc = new jsPDF();
    const pageWidth = doc.internal.pageSize.getWidth();
    
    // Header
    doc.setFontSize(24);
    doc.setTextColor(16, 185, 129); // emerald-500
    doc.text('INVOICE', 20, 25);
    
    doc.setFontSize(12);
    doc.setTextColor(100);
    doc.text(`#${invoice.invoice_number}`, 20, 33);
    
    // Company info (right side)
    doc.setFontSize(10);
    doc.setTextColor(60);
    doc.text('Chrono Time Tracking', pageWidth - 20, 25, { align: 'right' });
    
    // Client info
    doc.setFontSize(11);
    doc.setTextColor(40);
    doc.text('Bill To:', 20, 50);
    doc.setFontSize(12);
    doc.setTextColor(0);
    doc.text(invoice.client_name, 20, 58);
    if (invoice.client_email) {
      doc.setFontSize(10);
      doc.setTextColor(100);
      doc.text(invoice.client_email, 20, 65);
    }
    
    // Dates
    doc.setFontSize(10);
    doc.setTextColor(100);
    doc.text(`Date: ${formatDate(invoice.created_date)}`, pageWidth - 20, 50, { align: 'right' });
    doc.text(`Due: ${formatDate(invoice.due_date || '')}`, pageWidth - 20, 57, { align: 'right' });
    doc.text(`Status: ${invoice.status || 'Draft'}`, pageWidth - 20, 64, { align: 'right' });
    
    // Items table
    const tableData = invoice.items?.map(item => [
      item.description || 'Work Session',
      `${item.hours.toFixed(1)}h`,
      formatCurrency(item.rate),
      formatCurrency(item.amount)
    ]) || [['No items', '-', '-', '-']];
    
    autoTable(doc, {
      startY: 80,
      head: [['Description', 'Hours', 'Rate', 'Amount']],
      body: tableData,
      theme: 'striped',
      headStyles: { fillColor: [16, 185, 129], textColor: 255 },
      styles: { fontSize: 10 },
      columnStyles: {
        0: { cellWidth: 80 },
        1: { cellWidth: 30, halign: 'center' },
        2: { cellWidth: 35, halign: 'right' },
        3: { cellWidth: 35, halign: 'right' }
      }
    });
    
    // Totals - position below table with proper spacing
    const finalY = ((doc as any).lastAutoTable?.finalY ?? 100) + 15;
    
    doc.setFontSize(10);
    doc.setTextColor(100);
    doc.text('Subtotal:', pageWidth - 70, finalY);
    doc.setTextColor(0);
    doc.text(formatCurrency(invoice.subtotal), pageWidth - 20, finalY, { align: 'right' });
    
    let currentY = finalY;
    if (invoice.tax_amount > 0) {
      currentY += 8;
      doc.setTextColor(100);
      doc.text(`Tax (${(invoice.tax_rate || 0) * 100}%):`, pageWidth - 70, currentY);
      doc.setTextColor(0);
      doc.text(formatCurrency(invoice.tax_amount), pageWidth - 20, currentY, { align: 'right' });
    }
    
    currentY += 12;
    doc.setFontSize(14);
    doc.setTextColor(16, 185, 129);
    doc.text('Total:', pageWidth - 70, currentY);
    doc.text(formatCurrency(invoice.total), pageWidth - 20, currentY, { align: 'right' });
    
    // Notes
    if (invoice.notes) {
      doc.setFontSize(10);
      doc.setTextColor(100);
      doc.text('Notes:', 20, currentY + 20);
      doc.setTextColor(60);
      doc.text(invoice.notes, 20, currentY + 28);
    }
    
    // Footer
    doc.setFontSize(9);
    doc.setTextColor(150);
    doc.text('Thank you for your business!', pageWidth / 2, 280, { align: 'center' });
    doc.text('Generated by Chrono', pageWidth / 2, 286, { align: 'center' });
    
    return doc;
  }

  function downloadPDF(invoice: Invoice) {
    try {
      const doc = generatePDF(invoice);
      doc.save(`Invoice_${invoice.invoice_number}.pdf`);
      showStatus('PDF downloaded!', 'success');
    } catch (e) {
      console.error('PDF generation failed:', e);
      showStatus('Failed to generate PDF', 'error');
    }
  }

  function printInvoice(invoice: Invoice) {
    try {
      const doc = generatePDF(invoice);
      // Create blob and open in new window for printing
      const blob = doc.output('blob');
      const blobUrl = URL.createObjectURL(blob);
      
      const printWindow = window.open(blobUrl, '_blank');
      if (printWindow) {
        printWindow.onload = () => {
          setTimeout(() => {
            printWindow.print();
          }, 250);
        };
        showStatus('Opening print dialog...', 'success');
      } else {
        // Fallback: just download the PDF
        downloadPDF(invoice);
        showStatus('Print blocked - PDF downloaded instead', 'success');
      }
    } catch (e) {
      console.error('Print failed:', e);
      showStatus('Failed to print invoice: ' + (e as Error).message, 'error');
    }
  }

  function sendEmail(invoice: Invoice) {
    const subject = encodeURIComponent(`Invoice #${invoice.invoice_number} from Chrono`);
    const body = encodeURIComponent(
`Dear ${invoice.client_name},

Please find attached Invoice #${invoice.invoice_number}.

Invoice Details:
- Amount: ${formatCurrency(invoice.total)}
- Due Date: ${formatDate(invoice.due_date || '')}
${invoice.notes ? `\nNotes: ${invoice.notes}` : ''}

Thank you for your business!

Best regards`
    );
    
    const mailtoUrl = `mailto:${invoice.client_email || ''}?subject=${subject}&body=${body}`;
    window.open(mailtoUrl, '_blank');
    showStatus('Opening email client...', 'success');
    
    // Also generate PDF for manual attachment
    downloadPDF(invoice);
  }

  function exportJSON(invoice: Invoice) {
    const dataStr = JSON.stringify(invoice, null, 2);
    const blob = new Blob([dataStr], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `Invoice_${invoice.invoice_number}.json`;
    a.click();
    URL.revokeObjectURL(url);
    showStatus('JSON exported!', 'success');
  }

  function exportCSV(invoice: Invoice) {
    let csv = 'Description,Hours,Rate,Amount\n';
    invoice.items?.forEach(item => {
      csv += `"${item.description || 'Work Session'}",${item.hours},${item.rate},${item.amount}\n`;
    });
    csv += `\n,,Subtotal,${invoice.subtotal}\n`;
    csv += `,,Tax,${invoice.tax_amount}\n`;
    csv += `,,Total,${invoice.total}\n`;
    
    const blob = new Blob([csv], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `Invoice_${invoice.invoice_number}.csv`;
    a.click();
    URL.revokeObjectURL(url);
    showStatus('CSV exported!', 'success');
  }

  function toggleSession(id: number) {
    if (selectedSessionIds.includes(id)) {
      selectedSessionIds = selectedSessionIds.filter(s => s !== id);
    } else {
      selectedSessionIds = [...selectedSessionIds, id];
    }
  }

  function selectAllSessions() {
    selectedSessionIds = uninvoicedSessions.map(s => s.id);
  }

  function formatCurrency(amount: number): string {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(amount || 0);
  }

  function formatDate(dateStr: string): string {
    if (!dateStr) return 'N/A';
    return new Date(dateStr).toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
  }

  function getStatusColor(status: string): string {
    switch (status?.toLowerCase()) {
      case 'draft': return 'bg-gray-100 text-gray-700';
      case 'sent': return 'bg-blue-100 text-blue-700';
      case 'paid': return 'bg-green-100 text-green-700';
      case 'overdue': return 'bg-red-100 text-red-700';
      default: return 'bg-gray-100 text-gray-700';
    }
  }

  function getStatusIcon(status: string) {
    switch (status?.toLowerCase()) {
      case 'draft': return FileText;
      case 'sent': return Send;
      case 'paid': return CheckCircle;
      case 'overdue': return Clock;
      default: return FileText;
    }
  }

  function calculateSelectedTotal(): number {
    return uninvoicedSessions
      .filter(s => selectedSessionIds.includes(s.id))
      .reduce((sum, s) => sum + calculateSessionPay(s), 0);
  }

  function calculateSelectedHours(): number {
    return uninvoicedSessions
      .filter(s => selectedSessionIds.includes(s.id))
      .reduce((sum, s) => sum + s.hours, 0);
  }

  function calculateSessionPay(session: WorkSession): number {
    // Only calculate pay if pay_type is explicitly set
    if (session.pay_type === 'Fixed' && session.fixed_amount) {
      return session.fixed_amount;
    }
    if (session.pay_type === 'Hourly' && session.hourly_rate && session.hours) {
      return session.hourly_rate * session.hours;
    }
    // No pay type set = unpaid session
    return 0;
  }
</script>

<div class="p-6 space-y-6 max-w-6xl mx-auto">
  <!-- Status Toast -->
  {#if statusMessage}
    <div class="fixed top-4 right-4 z-[60] px-4 py-3 rounded-lg shadow-lg {statusType === 'success' ? 'bg-green-500' : 'bg-red-500'} text-white flex items-center gap-2 animate-fade-in">
      {#if statusType === 'success'}
        <CheckCircle size={18} />
      {:else}
        <X size={18} />
      {/if}
      {statusMessage}
    </div>
  {/if}

  <!-- Header -->
  <div class="flex items-center justify-between mb-6">
    <div class="flex items-center gap-3">
      <div class="p-2 bg-emerald-100 rounded-lg">
        <FileText size={24} class="text-emerald-600" />
      </div>
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Invoices</h1>
        <p class="text-gray-500 text-sm">Generate, send, and track client invoices</p>
      </div>
    </div>

    <button
      on:click={openCreateDialog}
      class="flex items-center gap-2 px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors"
    >
      <Plus size={18} />
      New Invoice
    </button>
  </div>

  {#if loading}
    <div class="card p-12 bg-white rounded-xl shadow-sm border text-center">
      <div class="animate-spin w-8 h-8 border-2 border-emerald-600 border-t-transparent rounded-full mx-auto mb-3"></div>
      <p class="text-gray-500">Loading invoices...</p>
    </div>
  {:else}
    <!-- Stats -->
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
      <div class="card p-4 bg-white rounded-xl shadow-sm border">
        <p class="text-xs text-gray-500 uppercase tracking-wide">Total Invoices</p>
        <p class="text-2xl font-bold text-gray-900">{invoices.length}</p>
      </div>
      <div class="card p-4 bg-white rounded-xl shadow-sm border">
        <p class="text-xs text-gray-500 uppercase tracking-wide">Total Billed</p>
        <p class="text-2xl font-bold text-emerald-600">
          {formatCurrency(invoices.reduce((sum, i) => sum + (i.total || 0), 0))}
        </p>
      </div>
      <div class="card p-4 bg-white rounded-xl shadow-sm border">
        <p class="text-xs text-gray-500 uppercase tracking-wide">Paid</p>
        <p class="text-2xl font-bold text-green-600">
          {formatCurrency(invoices.filter(i => i.status?.toLowerCase() === 'paid').reduce((sum, i) => sum + (i.total || 0), 0))}
        </p>
      </div>
      <div class="card p-4 bg-white rounded-xl shadow-sm border">
        <p class="text-xs text-gray-500 uppercase tracking-wide">Pending</p>
        <p class="text-2xl font-bold text-amber-600">
          {formatCurrency(invoices.filter(i => i.status?.toLowerCase() !== 'paid').reduce((sum, i) => sum + (i.total || 0), 0))}
        </p>
      </div>
    </div>

    <!-- Invoice List -->
    {#if invoices.length > 0}
      <div class="card bg-white rounded-xl shadow-sm border overflow-hidden">
        <table class="w-full">
          <thead class="bg-gray-50 border-b">
            <tr>
              <th class="text-left px-4 py-3 text-sm font-medium text-gray-500">Invoice</th>
              <th class="text-left px-4 py-3 text-sm font-medium text-gray-500">Client</th>
              <th class="text-left px-4 py-3 text-sm font-medium text-gray-500">Date</th>
              <th class="text-left px-4 py-3 text-sm font-medium text-gray-500">Amount</th>
              <th class="text-left px-4 py-3 text-sm font-medium text-gray-500">Status</th>
              <th class="text-right px-4 py-3 text-sm font-medium text-gray-500">Actions</th>
            </tr>
          </thead>
          <tbody class="divide-y">
            {#each invoices as invoice}
              <tr class="hover:bg-gray-50">
                <td class="px-4 py-3">
                  <span class="font-medium text-gray-900">#{invoice.invoice_number}</span>
                </td>
                <td class="px-4 py-3">
                  <div>
                    <p class="font-medium text-gray-900">{invoice.client_name}</p>
                    {#if invoice.client_email}
                      <p class="text-xs text-gray-500">{invoice.client_email}</p>
                    {/if}
                  </div>
                </td>
                <td class="px-4 py-3 text-sm text-gray-600">{formatDate(invoice.created_date)}</td>
                <td class="px-4 py-3 font-semibold text-gray-900">{formatCurrency(invoice.total)}</td>
                <td class="px-4 py-3">
                  <span class="inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs font-medium capitalize {getStatusColor(invoice.status)}">
                    <svelte:component this={getStatusIcon(invoice.status)} size={12} />
                    {invoice.status || 'Draft'}
                  </span>
                </td>
                <td class="px-4 py-3">
                  <div class="flex items-center justify-end gap-1">
                    <!-- Mark as Paid -->
                    {#if invoice.status?.toLowerCase() !== 'paid'}
                      <button
                        on:click={() => handleStatusChange(invoice, 'Paid')}
                        class="p-1.5 text-green-600 hover:bg-green-50 rounded transition-colors"
                        title="Mark as Paid"
                      >
                        <CheckCircle size={16} />
                      </button>
                    {/if}
                    
                    <!-- Send Email -->
                    <button
                      on:click={() => sendEmail(invoice)}
                      class="p-1.5 text-blue-600 hover:bg-blue-50 rounded transition-colors"
                      title="Send via Email"
                    >
                      <Mail size={16} />
                    </button>
                    
                    <!-- Download PDF -->
                    <button
                      on:click={() => downloadPDF(invoice)}
                      class="p-1.5 text-purple-600 hover:bg-purple-50 rounded transition-colors"
                      title="Download PDF"
                    >
                      <FileDown size={16} />
                    </button>
                    
                    <!-- Print -->
                    <button
                      on:click={() => printInvoice(invoice)}
                      class="p-1.5 text-gray-600 hover:bg-gray-100 rounded transition-colors"
                      title="Print"
                    >
                      <Printer size={16} />
                    </button>
                    
                    <!-- View Details -->
                    <button
                      on:click={() => selectedInvoice = invoice}
                      class="p-1.5 text-gray-600 hover:bg-gray-100 rounded transition-colors"
                      title="View Details"
                    >
                      <Eye size={16} />
                    </button>
                    
                    <!-- Edit -->
                    <button
                      on:click={() => openEditDialog(invoice)}
                      class="p-1.5 text-amber-600 hover:bg-amber-50 rounded transition-colors"
                      title="Edit"
                    >
                      <Edit size={16} />
                    </button>
                    
                    <!-- Delete -->
                    <button
                      on:click={() => handleDelete(invoice)}
                      class="p-1.5 text-red-600 hover:bg-red-50 rounded transition-colors"
                      title="Delete"
                    >
                      <Trash2 size={16} />
                    </button>
                  </div>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {:else}
      <div class="card p-12 bg-gray-50 rounded-xl border border-dashed text-center">
        <FileText size={48} class="text-gray-400 mx-auto mb-3" />
        <p class="text-gray-500 font-medium">No invoices yet</p>
        <p class="text-sm text-gray-400 mb-4">Create your first invoice from tracked sessions</p>
        <button
          on:click={openCreateDialog}
          class="inline-flex items-center gap-2 px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700"
        >
          <Plus size={18} />
          Create Invoice
        </button>
      </div>
    {/if}
  {/if}
</div>

<!-- Create Invoice Dialog -->
{#if showCreateDialog}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
    <div class="bg-white rounded-xl shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
      <div class="flex items-center justify-between p-4 border-b">
        <h2 class="text-lg font-semibold">Create Invoice</h2>
        <button on:click={() => showCreateDialog = false} class="p-1 hover:bg-gray-100 rounded">
          <X size={20} />
        </button>
      </div>

      <div class="p-4 space-y-4">
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label for="clientName" class="block text-sm font-medium text-gray-700 mb-1">Client Name *</label>
            <input
              id="clientName"
              type="text"
              bind:value={clientName}
              class="w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-emerald-500"
              placeholder="Client or company name"
            />
          </div>
          <div>
            <label for="clientEmail" class="block text-sm font-medium text-gray-700 mb-1">Client Email</label>
            <input
              id="clientEmail"
              type="email"
              bind:value={clientEmail}
              class="w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-emerald-500"
              placeholder="client@example.com"
            />
          </div>
        </div>

        <div>
          <label for="dueDate" class="block text-sm font-medium text-gray-700 mb-1">Due Date</label>
          <input
            id="dueDate"
            type="date"
            bind:value={dueDate}
            class="w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-emerald-500"
          />
        </div>

        <div>
          <div class="flex items-center justify-between mb-2">
            <span class="block text-sm font-medium text-gray-700">Select Sessions *</span>
            <button
              on:click={selectAllSessions}
              class="text-sm text-emerald-600 hover:underline"
            >
              Select All
            </button>
          </div>

          {#if uninvoicedSessions.length > 0}
            <div class="border rounded-lg max-h-48 overflow-y-auto">
              {#each uninvoicedSessions as session}
                <label class="flex items-center gap-3 p-3 hover:bg-gray-50 cursor-pointer border-b last:border-b-0">
                  <input
                    type="checkbox"
                    checked={selectedSessionIds.includes(session.id)}
                    on:change={() => toggleSession(session.id)}
                    class="rounded text-emerald-600 focus:ring-emerald-500"
                  />
                  <div class="flex-1 min-w-0">
                    <p class="font-medium text-gray-900 truncate">{session.project_name || 'Unnamed'}</p>
                    <p class="text-xs text-gray-500">{formatDate(session.date)} • {session.hours}h</p>
                  </div>
                  <span class="font-medium text-gray-900">{formatCurrency(calculateSessionPay(session))}</span>
                </label>
              {/each}
            </div>
          {:else}
            <div class="p-4 bg-gray-50 rounded-lg text-center text-sm text-gray-500">
              No uninvoiced sessions available
            </div>
          {/if}
        </div>

        {#if selectedSessionIds.length > 0}
          <div class="p-3 bg-emerald-50 rounded-lg">
            <div class="flex justify-between text-sm">
              <span class="text-emerald-700">{selectedSessionIds.length} sessions • {calculateSelectedHours().toFixed(1)}h</span>
              <span class="font-bold text-emerald-700">{formatCurrency(calculateSelectedTotal())}</span>
            </div>
          </div>
        {/if}

        <div>
          <label for="notes" class="block text-sm font-medium text-gray-700 mb-1">Notes</label>
          <textarea
            id="notes"
            bind:value={notes}
            class="w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-emerald-500"
            rows="2"
            placeholder="Additional notes for the invoice"
          ></textarea>
        </div>
      </div>

      <div class="flex justify-end gap-3 p-4 border-t bg-gray-50">
        <button
          on:click={() => showCreateDialog = false}
          class="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg"
        >
          Cancel
        </button>
        <button
          on:click={handleCreate}
          disabled={!clientName || selectedSessionIds.length === 0}
          class="px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Create Invoice
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Edit Invoice Dialog -->
{#if showEditDialog && editingInvoice}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
    <div class="bg-white rounded-xl shadow-xl max-w-md w-full">
      <div class="flex items-center justify-between p-4 border-b">
        <h2 class="text-lg font-semibold">Edit Invoice #{editingInvoice.invoice_number}</h2>
        <button on:click={() => showEditDialog = false} class="p-1 hover:bg-gray-100 rounded">
          <X size={20} />
        </button>
      </div>

      <div class="p-4 space-y-4">
        <div>
          <span class="block text-sm font-medium text-gray-700 mb-2">Status</span>
          <div class="flex flex-wrap gap-2">
            {#each ['Draft', 'Sent', 'Paid', 'Overdue'] as status}
              <button
                on:click={() => editingInvoice && handleStatusChange(editingInvoice, status)}
                class="px-3 py-1.5 text-sm rounded-lg border transition-colors {editingInvoice.status?.toLowerCase() === status.toLowerCase() ? 'bg-emerald-600 text-white border-emerald-600' : 'hover:bg-gray-50'}"
              >
                {status}
              </button>
            {/each}
          </div>
        </div>

        <div class="p-3 bg-gray-50 rounded-lg">
          <div class="text-sm text-gray-600">
            <p><strong>Client:</strong> {editingInvoice.client_name}</p>
            {#if editingInvoice.client_email}
              <p><strong>Email:</strong> {editingInvoice.client_email}</p>
            {/if}
            <p><strong>Amount:</strong> {formatCurrency(editingInvoice.total)}</p>
            <p><strong>Created:</strong> {formatDate(editingInvoice.created_date)}</p>
            <p><strong>Due:</strong> {formatDate(editingInvoice.due_date || '')}</p>
          </div>
        </div>
      </div>

      <div class="flex justify-end gap-3 p-4 border-t bg-gray-50">
        <button
          on:click={() => showEditDialog = false}
          class="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg"
        >
          Close
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Invoice Detail Dialog -->
{#if selectedInvoice}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
    <div class="bg-white rounded-xl shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
      <div class="flex items-center justify-between p-4 border-b">
        <h2 class="text-lg font-semibold">Invoice #{selectedInvoice.invoice_number}</h2>
        <button on:click={() => selectedInvoice = null} class="p-1 hover:bg-gray-100 rounded">
          <X size={20} />
        </button>
      </div>

      <div class="p-4 space-y-4">
        <div class="flex items-center justify-between">
          <div>
            <p class="font-medium text-gray-900">{selectedInvoice.client_name}</p>
            {#if selectedInvoice.client_email}
              <p class="text-sm text-gray-500">{selectedInvoice.client_email}</p>
            {/if}
          </div>
          <span class="px-3 py-1 rounded-full text-sm font-medium capitalize {getStatusColor(selectedInvoice.status)}">
            {selectedInvoice.status || 'Draft'}
          </span>
        </div>

        <div class="grid grid-cols-2 gap-4 text-sm">
          <div>
            <p class="text-gray-500">Created</p>
            <p class="font-medium">{formatDate(selectedInvoice.created_date)}</p>
          </div>
          {#if selectedInvoice.due_date}
            <div>
              <p class="text-gray-500">Due Date</p>
              <p class="font-medium">{formatDate(selectedInvoice.due_date)}</p>
            </div>
          {/if}
        </div>

        <!-- Items -->
        {#if selectedInvoice.items && selectedInvoice.items.length > 0}
          <div class="border rounded-lg overflow-hidden">
            <table class="w-full text-sm">
              <thead class="bg-gray-50">
                <tr>
                  <th class="text-left px-3 py-2 font-medium text-gray-500">Description</th>
                  <th class="text-right px-3 py-2 font-medium text-gray-500">Hours</th>
                  <th class="text-right px-3 py-2 font-medium text-gray-500">Rate</th>
                  <th class="text-right px-3 py-2 font-medium text-gray-500">Amount</th>
                </tr>
              </thead>
              <tbody class="divide-y">
                {#each selectedInvoice.items as item}
                  <tr>
                    <td class="px-3 py-2">{item.description || 'Work Session'}</td>
                    <td class="px-3 py-2 text-right">{item.hours.toFixed(1)}h</td>
                    <td class="px-3 py-2 text-right">{formatCurrency(item.rate)}</td>
                    <td class="px-3 py-2 text-right font-medium">{formatCurrency(item.amount)}</td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}

        <div class="border-t pt-4">
          <div class="flex justify-between text-sm mb-1">
            <span class="text-gray-500">Subtotal</span>
            <span class="font-medium">{formatCurrency(selectedInvoice.subtotal)}</span>
          </div>
          {#if selectedInvoice.tax_amount > 0}
            <div class="flex justify-between text-sm mb-1">
              <span class="text-gray-500">Tax ({(selectedInvoice.tax_rate || 0) * 100}%)</span>
              <span class="font-medium">{formatCurrency(selectedInvoice.tax_amount)}</span>
            </div>
          {/if}
          <div class="flex justify-between text-lg font-bold mt-2 pt-2 border-t">
            <span>Total</span>
            <span class="text-emerald-600">{formatCurrency(selectedInvoice.total)}</span>
          </div>
        </div>

        {#if selectedInvoice.notes}
          <div class="p-3 bg-gray-50 rounded-lg text-sm text-gray-600">
            <p class="font-medium text-gray-700 mb-1">Notes:</p>
            {selectedInvoice.notes}
          </div>
        {/if}
      </div>

      <div class="flex flex-wrap justify-end gap-2 p-4 border-t bg-gray-50">
        <button
          on:click={() => selectedInvoice && sendEmail(selectedInvoice)}
          class="flex items-center gap-2 px-3 py-2 text-sm text-blue-600 hover:bg-blue-50 rounded-lg"
        >
          <Mail size={16} />
          Email
        </button>
        <button
          on:click={() => selectedInvoice && downloadPDF(selectedInvoice)}
          class="flex items-center gap-2 px-3 py-2 text-sm text-purple-600 hover:bg-purple-50 rounded-lg"
        >
          <FileDown size={16} />
          PDF
        </button>
        <button
          on:click={() => selectedInvoice && printInvoice(selectedInvoice)}
          class="flex items-center gap-2 px-3 py-2 text-sm text-gray-600 hover:bg-gray-100 rounded-lg"
        >
          <Printer size={16} />
          Print
        </button>
        <button
          on:click={() => selectedInvoice && exportCSV(selectedInvoice)}
          class="flex items-center gap-2 px-3 py-2 text-sm text-green-600 hover:bg-green-50 rounded-lg"
        >
          <Download size={16} />
          CSV
        </button>
        <button
          on:click={() => selectedInvoice && exportJSON(selectedInvoice)}
          class="flex items-center gap-2 px-3 py-2 text-sm text-amber-600 hover:bg-amber-50 rounded-lg"
        >
          <Share2 size={16} />
          JSON
        </button>
        <button
          on:click={() => selectedInvoice = null}
          class="px-4 py-2 text-sm text-gray-700 hover:bg-gray-200 rounded-lg"
        >
          Close
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  @keyframes fade-in {
    from { opacity: 0; transform: translateY(-10px); }
    to { opacity: 1; transform: translateY(0); }
  }
  .animate-fade-in {
    animation: fade-in 0.3s ease-out;
  }
</style>
