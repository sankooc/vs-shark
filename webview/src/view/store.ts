// userStore.ts
import { create } from "zustand";
import { immer } from "zustand/middleware/immer";
import { onMessage, emitMessage } from "../core/connect";
import { _log } from "./util";
import { ComMessage, ComType } from "../core/common";

// interface User {
//   id: string
//   name: string
//   email: string
//   age: number
// }

// interface UserState {
//   users: User[]
//   loading: boolean
//   error: string | null
//   fetchUsers: () => Promise<void>
//   addUser: (user: Omit<User, 'id'>) => void
//   updateUser: (id: string, user: Partial<User>) => void
//   deleteUser: (id: string) => void
// }

interface PcapState {
  filename: string;
  size: number;
  loading: boolean;
  status: string;
  sendReady: () => void;
  request: () => void;
}

export const useStore = create<PcapState>()((set) => {
  _log("create pcap store");
  onMessage("message", (e: any) => {
    const { type, body, id } = e.data;
    _log(type, body, id);
    switch (type) {
      case ComType.SERVER_REDAY: {
        //   emitMessage(ComMessage.new(ComType.CLIENT_REDAY, Date.now()));
        break;
      }
    }
  });
  return {
    filename: "",
    size: 0,
    loading: false,
    status: "",
    sendReady: () => {
      emitMessage(ComMessage.new(ComType.CLIENT_REDAY, Date.now()));
    },
    request: () => {
      emitMessage(new ComMessage(ComType.REQUEST, ""));
    },
  };
});

// export const useUserStore = create<UserState>()(
//   immer(
//     (set, get) => ({
//       users: [],
//       loading: false,
//       error: null,

//       // 异步获取用户
//       fetchUsers: async () => {
//         set({ loading: true, error: null })
//         try {
//           const response = await fetch('https://api.example.com/users')
//           const data = await response.json()
//           set({ users: data, loading: false })
//         } catch (err) {
//           set({ error: (err as Error).message, loading: false })
//         }
//       },

//       // 添加用户（使用 Immer 简化不可变更新）
//       addUser: (user) => {
//         set((state) => {
//           state.users.push({
//             ...user,
//             id: Math.random().toString(36).substring(2, 9)
//           })
//         })
//       },

//       // 更新用户
//       updateUser: (id, updates) => {
//         set((state) => {
//           const index = state.users.findIndex((u: User) => u.id === id)
//           if (index !== -1) {
//             state.users[index] = { ...state.users[index], ...updates }
//           }
//         })
//       },

//       // 删除用户
//       deleteUser: (id) => {
//         set((state) => {
//           state.users = state.users.filter((user: User) => user.id !== id)
//         })
//       }
//     })
//   )
// )
