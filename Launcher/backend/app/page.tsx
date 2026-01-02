export default function Home() {
  return (
    <main style={{ 
      display: 'flex', 
      flexDirection: 'column', 
      alignItems: 'center', 
      justifyContent: 'center', 
      minHeight: '100vh',
      fontFamily: 'system-ui, sans-serif',
    }}>
      <h1 style={{ fontSize: '2rem', marginBottom: '1rem' }}>⏱️ Chrono API</h1>
      <p style={{ color: '#666' }}>License management backend</p>
      <p style={{ color: '#999', fontSize: '0.875rem', marginTop: '2rem' }}>
        Looking for the app? Visit <a href="https://chrono.app" style={{ color: '#4F46E5' }}>chrono.app</a>
      </p>
    </main>
  );
}
