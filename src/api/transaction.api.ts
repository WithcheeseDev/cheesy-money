import { invoke } from "@tauri-apps/api/core";
import type { Transaction } from "../types/transaction";

export function getTransactions(): Promise<Transaction[]> {
  return invoke<Transaction[]>("get_transactions");
}
