# 📜 Stellar Daily Quote Saver DApp

**Stellar Daily Quote Saver** is a decentralized application (DApp) built on the Stellar blockchain using the Soroban SDK.  
This smart contract allows users to store, manage, and organize inspirational quotes in a secure and transparent way directly on-chain.

---

## 🚀 Project Description

Daily Quote Saver is a simple yet meaningful blockchain-based application that enables users to:

- Save personal or inspirational quotes  
- Assign authors to each quote  
- Mark favorite quotes  
- Track when quotes were created  
- Manage data without relying on centralized storage  

All data is stored inside the smart contract, ensuring persistence, transparency, and integrity.

---

## 🎯 Project Vision

This project aims to demonstrate how even simple personal tools can benefit from decentralization:

- **Data Ownership** → Users fully control their stored quotes  
- **Transparency** → All data interactions are verifiable on-chain  
- **Simplicity** → Minimal yet functional use of blockchain technology  
- **Learning Focus** → Designed as a foundational project for understanding smart contracts  

---

## ✨ Key Features

### 1. 📝 Create Quote
- Add a new quote with text and author  
- Automatically generates a unique ID  
- Records timestamp of creation  

### 2. 📚 View Quotes
- Retrieve all saved quotes  
- Structured format for easy frontend integration  

### 3. ❌ Delete Quote
- Remove quotes using their unique ID  
- Updates storage immediately  

### 4. ⭐ Favorite System
- Mark or unmark quotes as favorite  
- Helps users highlight important quotes  

### 5. ⏱ Timestamp Tracking
- Each quote stores the creation time  
- Enables sorting or history tracking  

---

## 🧱 Data Structure

Each quote contains:

- `id` → unique identifier  
- `text` → quote content  
- `author` → quote author  
- `is_favorite` → favorite status  
- `created_at` → timestamp  

---

## ⚙️ Smart Contract Functions

- `create_quote(text, author)`  
- `get_quotes()`  
- `delete_quote(id)`  
- `toggle_favorite(id)`  

---

## 🔗 Contract Details

- **Network**: Stellar (Soroban)  
- **Language**: Rust  
- **SDK**: Soroban SDK  

---

## 🛠 Tech Stack

- Rust Programming Language  
- Soroban Smart Contract SDK  
- Stellar Blockchain  

---

## 📈 Future Improvements

### Short-Term
- Search quotes by keyword  
- Filter favorite quotes  
- Improve ID generation (avoid randomness collision)  

### Mid-Term
- Add categories/tags  
- Simple frontend integration (HTML/JS)  
- Sorting by date or favorite  

### Long-Term
- Encryption for private quotes  
- Multi-user support  
- Decentralized frontend (IPFS)  

---

## ▶️ Getting Started

1. Deploy the smart contract to the Soroban network  
2. Interact using available functions:
   - Create quote  
   - View quotes  
   - Toggle favorite  
   - Delete quote  

---

## 🧠 Notes

This project is designed as a learning-focused DApp to understand:

- Smart contract storage  
- Data structures in blockchain  
- State management using Soroban  

---

## 📌 Conclusion

Daily Quote Saver shows that even a simple idea can be enhanced using blockchain principles.  
It focuses on clarity, functionality, and foundational understanding rather than unnecessary complexity.

---

**"Simple, but meaningful."**