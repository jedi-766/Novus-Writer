import { create } from 'zustand';

export interface UIStore {
  // Sidebar
  isSidebarOpen: boolean;
  sidebarWidth: number;
  
  // Ribbon
  activeRibbonTab: string;
  isRibbonMinimized: boolean;
  
  // Modals
  activeModal: string | null;
  
  // Actions
  toggleSidebar: () => void;
  setSidebarWidth: (width: number) => void;
  setActiveRibbonTab: (tab: string) => void;
  toggleRibbonMinimized: () => void;
  setActiveModal: (modal: string | null) => void;
}

export const useUIStore = create<UIStore>((set) => ({
  isSidebarOpen: true,
  sidebarWidth: 250,
  activeRibbonTab: 'home',
  isRibbonMinimized: false,
  activeModal: null,
  
  toggleSidebar: () => set((state) => ({ isSidebarOpen: !state.isSidebarOpen })),
  setSidebarWidth: (width: number) => set({ sidebarWidth: width }),
  setActiveRibbonTab: (tab: string) => set({ activeRibbonTab: tab }),
  toggleRibbonMinimized: () => set((state) => ({ isRibbonMinimized: !state.isRibbonMinimized })),
  setActiveModal: (modal: string | null) => set({ activeModal: modal }),
}));
