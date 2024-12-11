import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";

import "./App.css";
// PrimeReact
import React from 'react';
import { TabMenu } from 'primereact/tabmenu';
// import component
import CAC40 from "./components/cac40";
import Global100ETF from "./components/global100";
import Bitcoin from "./components/bitcoin";
import SP500 from "./components/sp500";

function App() {

  const items = [
    { label: "CAC40", icon: "pi pi-chart-bar", component: <CAC40 /> },
    { label: "Global 100 ETF", icon: "pi pi-globe", component: <Global100ETF /> },
    { label: "BitCoin", icon: "pi pi-bitcoin", component: <Bitcoin /> },
    { label: "S&P 500", icon: "pi pi-dollar", component: <SP500 /> }
  ];
  const [activeIndex, setActiveIndex] = useState(0);

  return (
    <body>
    <header>
      <nav>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo" alt="Tauri logo" />
        </a>
        <h1>Exchange Dashboard</h1>
        <img src="src-tauri/icons/wifi.png" alt="wifi_logo" className="wifi-icon logo" />
      </nav>
    </header>

    <main>
      {/* Menu */}
      <section className="menu">
        <TabMenu
          model={items}
          activeIndex={activeIndex}
          onTabChange={(e) => setActiveIndex(e.index)}
        />
      </section>

      {/* Contenu dynamique basé sur l'onglet actif */}
      <section className="rander">{items[activeIndex].component}</section>
    </main>
  </body>
  );
}

export default App;