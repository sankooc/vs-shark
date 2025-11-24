import { createContext, useContext, ReactNode } from 'react';
import { StoreApi, useStore } from 'zustand';
import { PcapState } from '../share/common';

type StoreContextType = StoreApi<PcapState> | null;

const StoreContext = createContext<StoreContextType>(null);

interface StoreProviderProps {
  store: StoreApi<PcapState>;
  children: ReactNode;
}

export function StoreProvider({ store, children }: StoreProviderProps) {
  return (
    <StoreContext.Provider value={store}>
      {children}
    </StoreContext.Provider>
  );
}

export function usePcapStore<U>(
  selector: (state: PcapState) => U
): U {
  const store = useContext(StoreContext);
  if (!store) {
    throw new Error('no StoreProvider');
  }
  return useStore(store, selector);
}