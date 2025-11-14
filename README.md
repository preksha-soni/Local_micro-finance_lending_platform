🏦 Local Micro-Finance Lending Platform
A lightweight but powerful platform designed for small communities, SHGs, and micro-finance groups to manage borrowers, loans, EMI, and repayments with full transparency and ease of use.
Built for local deployment on laptops, college projects, or small organizations.

📌 Key Features
👤 Borrower Management
Add and update borrower profiles

Track personal details, KYC, and loan history

Search and filter borrowers

💸 Loan Management
Create loans with principal, tenure, and interest rate

Auto-calculate EMI, total interest, and loan end-date

Loan approval workflow

Track active, completed, and overdue loans

💰 Repayment Tracking
Record loan repayments

Auto-update remaining balance

Late fee / penalty handling (optional)

📊 Admin Dashboard
View daily collections

Total loan summary

Active borrowers / overdue loans

Profit and interest earned

🔐 Security
JWT-based authentication

Role-based access (Admin, Field Officer)

Secure DB connections and hashing

🏗️ Tech Stack
Layer	Technology
Backend	FastAPI / Node.js
Database	MongoDB / PostgreSQL
Frontend	React / HTML Templates
Auth	JWT Authentication
Deployment	Docker / Local Server
📂 Folder Structure
micro-finance-platform/
│
├── backend/
│   ├── app/
│   │   ├── main.py
│   │   ├── routes/
│   │   │   ├── borrower.py
│   │   │   ├── loan.py
│   │   │   ├── repayment.py
│   │   ├── models/
│   │   ├── schemas/
│   │   ├── database.py
│   └── requirements.txt
│
├── frontend/
│   ├── public/
│   ├── src/
│   │   ├── components/
│   │   ├── pages/
│   │   ├── services/
│   └── package.json
│
└── README.md
⚙️ Installation & Setup
1. Clone the repository
git clone https://github.com/your-username/micro-finance-platform.git
cd micro-finance-platform
🖥️ Backend Setup (FastAPI Example)
2. Install backend requirements
cd backend
pip install -r requirements.txt
3. Run server
uvicorn app.main:app --reload
Backend will run at:
👉 http://localhost:8000

🌐 Frontend Setup (React Example)
4. Install dependencies
cd frontend
npm install
5. Start frontend
npm start
Frontend will run at:
👉 http://localhost:3000

📘 API Endpoints Overview
Borrowers
Method	Endpoint	Description
POST	/borrowers/	Create borrower
GET	/borrowers/	Get all borrowers
GET	/borrowers/{id}	Get borrower details
PUT	/borrowers/{id}	Update borrower
Loans
Method	Endpoint	Description
POST	/loans/	Create new loan
GET	/loans/active	Active loans
GET	/loans/{id}	Loan details
Repayments
Method	Endpoint	Description
POST	/repayments/	Add repayment
GET	/repayments/loan/{id}	Loan repayment history
📊 EMI Formula (Used in Loan Calculations)
EMI = [P × R × (1+R)^N] / [(1+R)^N – 1]
Where:

P = Principal

R = Monthly interest rate

N = Tenure in months

🧪 Running Tests
pytest
🔐 Security Highlights
JWT token authentication

Password hashing using Bcrypt

Environment variables for DB credentials

CORS policy enforced

🚀 Future Enhancements
SMS/WhatsApp repayment reminders

Payment gateway auto-collection

Multi-branch microfinance operations

ML model for loan default prediction

Printable reports (PDF) for daily collections

🤝 Contributing
Pull requests are welcome.
For new features, create an issue to discuss the idea before implementation.
