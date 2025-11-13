#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Address, Symbol, String, symbol_short};

// Loan structure to track individual loan details
#[contracttype]
#[derive(Clone)]
pub struct Loan {
    pub loan_id: u64,
    pub borrower: Address,
    pub amount: u64,          // Loan amount in stroops
    pub issued_time: u64,     // Timestamp when loan was issued
    pub repaid: bool,         // Repayment status
    pub repaid_time: u64,     // Timestamp when loan was repaid
}

// Global statistics for all loans
#[contracttype]
#[derive(Clone)]
pub struct LoanStats {
    pub total_loans: u64,
    pub active_loans: u64,
    pub repaid_loans: u64,
    pub total_disbursed: u64,  // Total amount disbursed
}

// Storage keys
const LOAN_COUNT: Symbol = symbol_short!("L_COUNT");
const LOAN_STATS: Symbol = symbol_short!("L_STATS");

// Mapping loan_id to Loan struct
#[contracttype]
pub enum LoanBook {
    Loan(u64)
}

#[contract]
pub struct MicrofinanceContract;

#[contractimpl]
impl MicrofinanceContract {

    /// Issue a new loan to a borrower
    /// Returns the unique loan ID
    pub fn issue_loan(env: Env, borrower: Address, amount: u64) -> u64 {
        // Get current loan count and increment
        let mut loan_count: u64 = env.storage().instance().get(&LOAN_COUNT).unwrap_or(0);
        loan_count += 1;

        // Get current timestamp
        let time = env.ledger().timestamp();

        // Get current stats
        let mut stats = Self::view_loan_stats(env.clone());

        // Create new loan record
        let loan = Loan {
            loan_id: loan_count,
            borrower: borrower.clone(),
            amount,
            issued_time: time,
            repaid: false,
            repaid_time: 0,
        };

        // Update statistics
        stats.total_loans += 1;
        stats.active_loans += 1;
        stats.total_disbursed += amount;

        // Store loan data
        env.storage().instance().set(&LoanBook::Loan(loan_count), &loan);
        env.storage().instance().set(&LOAN_COUNT, &loan_count);
        env.storage().instance().set(&LOAN_STATS, &stats);

        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Loan issued - ID: {}, Amount: {}", loan_count, amount);

        loan_count
    }

    /// Mark a loan as repaid
    pub fn repay_loan(env: Env, loan_id: u64) {
        // Retrieve loan record
        let mut loan = Self::view_loan(env.clone(), loan_id);

        // Check if loan exists and is not already repaid
        if loan.loan_id == 0 {
            log!(&env, "Loan ID {} not found", loan_id);
            panic!("Loan not found");
        }

        if loan.repaid == true {
            log!(&env, "Loan ID {} already repaid", loan_id);
            panic!("Loan already repaid");
        }

        // Get current timestamp
        let time = env.ledger().timestamp();

        // Update loan status
        loan.repaid = true;
        loan.repaid_time = time;

        // Update statistics
        let mut stats = Self::view_loan_stats(env.clone());
        stats.active_loans -= 1;
        stats.repaid_loans += 1;

        // Store updated data
        env.storage().instance().set(&LoanBook::Loan(loan_id), &loan);
        env.storage().instance().set(&LOAN_STATS, &stats);

        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Loan ID {} marked as repaid", loan_id);
    }

    /// View details of a specific loan by ID
    pub fn view_loan(env: Env, loan_id: u64) -> Loan {
        let key = LoanBook::Loan(loan_id);

        env.storage().instance().get(&key).unwrap_or(Loan {
            loan_id: 0,
            borrower: Address::from_string(&String::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF")),
            amount: 0,
            issued_time: 0,
            repaid: false,
            repaid_time: 0,
        })
    }

    /// View overall loan statistics
    pub fn view_loan_stats(env: Env) -> LoanStats {
        env.storage().instance().get(&LOAN_STATS).unwrap_or(LoanStats {
            total_loans: 0,
            active_loans: 0,
            repaid_loans: 0,
            total_disbursed: 0,
        })
    }
}