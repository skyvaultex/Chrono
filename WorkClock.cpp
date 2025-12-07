#include <iostream>
#include <string>
#include <vector>
#include <limits>
#include <fstream>
#include <iomanip>


class WorkSession {

    private:

    double projectHours;
    std::string projectName;
    std::string projectDate;

    public:
    
    void setHours(double h){
        if(h <= 24 && h > 0) projectHours = h;
    }

    void setName(const std::string& n){
        if(!n.empty()) projectName = n;
    }

    void setDate(const std::string& d){
        if(!d.empty()) projectDate = d;
    }

    double getHours() const {return projectHours;}
    const std::string& getName() const {return projectName;}
    const std::string& getDate() const {return projectDate;}
};

void addSession(std::vector <WorkSession>& sessions) {

    bool adding = true;

    while(adding){
        
        std::string date, project;
        double hours = 0;

        std::cout << "The project date (YYYY-MM-DD): ";
        std::getline(std::cin, date);
        
        std::cout << "The project name: ";
        std::getline(std::cin, project);

        bool isHoursOk = false;

        while(!isHoursOk){
            std::cout << "How many hours worked on the session: ";
            std::cin >> hours;

            if (!std::cin || hours <= 0 || hours > 24){
                std::cin.clear();
                std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
                std::cout << "Invalid hours!\nNote that you should enter value more or equal to 0 and less than 24!\n";
            } else {
                std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
                isHoursOk = true;
            }
        }
        
    
        WorkSession newSession;
        newSession.setDate(date);
        newSession.setHours(hours);
        newSession.setName(project);
        sessions.push_back(newSession);

        char choice;
        std::cout << "Do you want to add another session? Type Y or N: ";
        std::cin >> choice;
        std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');

        if(choice == 'n' || choice == 'N'){
            adding = false;
        }
    }
}

std::vector<WorkSession> loadFromFile(){

    std::vector<WorkSession> list;
    std::string project, date;
    double hours;

    std::ifstream inFile("sessions.txt");
    if(!inFile){
        std::cout << "Couldn't load the sessions database." << std::endl;
        return list;
    }

    while(inFile >> date >> project >> hours){
        WorkSession session;
        session.setDate(date);
        session.setName(project);
        session.setHours(hours);
        list.push_back(session);
    }

    return list;
}

void saveToFile(const std::vector<WorkSession>& list){
    std::ofstream outFile ("sessions.txt");
    if(!outFile){
        std::cout << "There was an error while accessing the sessions database\n";
        return;
    }

    for(const auto& session : list){
        outFile << session.getDate() << " "
        << session.getName() << " "
        << session.getHours() << "\n";
    }
    outFile.close();
    std::cout << "Sessions saved.\n";
}

void showSessions(const std::vector<WorkSession>& list){

    std::cout << std::left 
    << std::setw(12) << "Date"
    << std::setw(20) << "Projects"
    << std::setw(8) << "Hours"
    << "\n--------------------------------------\n";


    for(const auto& session : list){
        std::cout 
        << std::setw(12) << session.getDate() 
        << std::setw(20) << session.getName() 
        << std::setw(8)  << session.getHours() 
        << std::endl;
    }
}

int main()
{
    std::vector<WorkSession> sessions = loadFromFile();

    bool running = true;

    while(running){

        std::cout << "\n============ WORK LOG MENU ============\n";
        std::cout << " 1) Add a new session\n";
        std::cout << " 2) Show all sessions\n";
        std::cout << " 3) Load sessions from file\n";
        std::cout << " 4) Save sessions to file\n";
        std::cout << " 5) Exit\n";
        std::cout << "=======================================\n";

        int choice;
        std::cout << "Choose an option: ";
        std::cin >> choice;
        std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');

        switch(choice){

            case 1:
                addSession(sessions);
                break;

            case 2:
                showSessions(sessions);
                break;

            case 3:
                sessions = loadFromFile();
                break;

            case 4:
                saveToFile(sessions);
                break;

            case 5:
                running = false;
                break;

            default:
                std::cout << "Wrong option.\n";
        }
    }
    return 0;
}
