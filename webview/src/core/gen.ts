export interface IProgressStatus {
  total: number;
  cursor: number;
}
export interface IListResult<T> {
  items: T[];
  total: number;
  start: number;
}
export interface IFrameInfo {
  index: number;
  time: number;
  source: string;
  dest: string;
  protocol: string;
  len: number;
  irtt: number;
  info: string;
  status: string;
}
