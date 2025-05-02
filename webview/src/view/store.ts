// userStore.ts
import { create } from "zustand";
import { onMessage, emitMessage } from "../core/connect";
import { _log } from "./util";
import { ComMessage, ComType, deserialize, PcapFile } from "../core/common";
import { IFrameInfo, IListResult, IProgressStatus } from "../core/gen";

interface PcapState {
  fileinfo?: PcapFile;
  progress?: IProgressStatus;
  frameResult?: IListResult<IFrameInfo>;
  sendReady: () => void;
  request: (data: any) => string;
}

export const useStore = create<PcapState>()((set) => {
  _log("create pcap store");
  onMessage("message", (e: any) => {
    const { type, body } = e.data;
    // _log(type, body, id);
    // console.log();
    switch (type) {
      case ComType.SERVER_REDAY: {
        //   emitMessage(ComMessage.new(ComType.CLIENT_REDAY, Date.now()));
        break;
      }
      case ComType.FILEINFO:
        const fileinfo = body as PcapFile;
        set((state) => ({ ...state, fileinfo: fileinfo }));
        break;
      case ComType.PRGRESS_STATUS:
        const progress = deserialize(body) as IProgressStatus;
        set((state) => ({ ...state, progress }));
        break;
      case ComType.FRAMES:
        const frameResult: IListResult<IFrameInfo> = deserialize(body);
        set((state) => ({ ...state, frameResult }));
        break;
    }
  });
  return {
    // filename: "",
    // size: 0,
    // loading: false,
    // status: "",
    sendReady: () => {
      emitMessage(ComMessage.new(ComType.CLIENT_REDAY, Date.now()));
    },
    request: (data: any): string => {
      const _req = new ComMessage(ComType.REQUEST, data);
      emitMessage(_req);
      return _req.id;
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
