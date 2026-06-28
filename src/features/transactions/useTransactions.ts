import { useEffect, useState } from "react";
import type { Transaction } from "../../types/transaction";
import { getTransactions } from "../../api/transaction.api";

interface UseTransactionsResult {
  transactions: Transaction[];
  error: string | null;
}

export function useTransactions(): UseTransactionsResult {
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    getTransactions()
      .then(setTransactions)
      .catch((err: unknown) => {
        console.error("Failed to fetch transactions:", err);
        setError(err instanceof Error ? err.message : String(err));
      });
  }, []);

  return { transactions, error };
}
