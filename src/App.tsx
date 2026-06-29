import { useState } from "react";
import SplashScreen from "./features/splash";
import Transactions from "./features/transactions";

function App() {
  const [showSplash, setShowSplash] = useState<boolean>(true);

  return showSplash ? (
    <SplashScreen onDismiss={() => setShowSplash(false)} />
  ) : (
    <main>
      <Transactions />
    </main>
  );
}

export default App;
