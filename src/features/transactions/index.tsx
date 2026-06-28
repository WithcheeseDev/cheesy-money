import { useTransactions } from "./useTransactions";

export default function Transactions() {
  const { transactions, error } = useTransactions();

  if (error) {
    return <p>Error: {error}</p>;
  }

  return (
    <ul>
      {transactions.map((t) => (
        <li key={t.id}>
          {t.description}: {t.amount}
        </li>
      ))}
    </ul>
  );
}
