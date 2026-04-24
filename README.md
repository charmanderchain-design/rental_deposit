# Project Title

Rental Deposit – A Soroban Smart Contract for Tenant Deposit Escrow on Stellar

## Project Vision

This project demonstrates a **rental deposit escrow smart contract** on Stellar using Soroban. It provides a transparent, on-chain mechanism for managing tenant security deposits:

- Tenant deposits are locked and tracked on-chain
- Landlord can release the deposit after tenancy ends
- Either party can raise a dispute
- Admin can resolve disputes by splitting funds proportionally

The goal is to provide a clean, auditable deposit management system built on Stellar's Soroban smart contract platform.

---

## Description

A Soroban smart contract that manages **tenant deposit escrow** on Stellar Testnet. The contract maintains a persistent on-chain record of deposits, their status, and supports release and dispute resolution workflows.

---

## Features

### 1. Tenant Deposit
- Tenant calls `deposit(tenant, landlord, amount)` to lock funds
- Deposits are tracked on-chain per tenant-landlord pair
- Funds remain locked until explicitly released or disputed

### 2. Landlord Release
- Landlord approves and releases deposit to themselves via `release_deposit(tenant, landlord)`
- Only works when deposit is in "held" status

### 3. Dispute Mechanism
- Either party can raise a dispute via `dispute(tenant, landlord)`
- Marks the deposit as "disputed", freezing automatic release

### 4. Admin Resolution
- Admin resolves disputed deposits via `resolve_dispute(landlord, tenant, landlord_share)`
- Funds split: `landlord_share` to landlord, remainder to tenant

### 5. On-chain Transparency
- Deposit status (`held`, `released`, `disputed`) stored permanently on Stellar
- Anyone can query `get_deposit(tenant, landlord)` to retrieve current state

---

## Contract

- **Network**: Stellar Testnet
- **Contract ID**: [CBF3FUGN5JH5CKJIMPOATHL53RTWPCIYZRH6GAS2LWJAVZQXSBUC4QOQ](https://stellar.expert/explorer/testnet/tx/594e21b8811c7898bb02b3424ec47c59c99997b4a51cabefefdc772e62711388)

![screenshot](https://i.ibb.co/jvL2LM4x/image.png)

---

## Future Scopes

### 1. Time-Locked Release
- Add a time delay before landlord can release deposit (e.g., 30 days after move-out)

### 2. Multi-Tenancy Support
- Allow multiple active deposits per tenant across different properties

### 3. Partial Releases
- Support incremental deposit releases for move-out inspections with deductions

### 4. Frontend dApp
- Build a React or plain HTML/JS interface for tenants, landlords, and admins to interact with the contract

### 5. Escrow with Native XLM
- Integrate Stellar native asset (XLM) transfer so deposits are actually held by the contract

### 6. Oracle Integration
- Use price oracles to handle deposit values in different currencies or stablecoins
---

## Profile

- **Name:** <!-- Fill github name -->
