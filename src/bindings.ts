         // This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

         export const commands = {
async getBranches() : Promise<Branches> {
return await TAURI_INVOKE("get_branches");
},
async addRepository(localPath: string, createdAt: string) : Promise<Result<null, null>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("add_repository", { localPath, createdAt }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getRepositories() : Promise<Result<Repository[], null>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("get_repositories") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getState() : Promise<Result<State, null>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("get_state") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async setOpenRepository(openRepository: number) : Promise<Result<null, null>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("set_open_repository", { openRepository }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
}
}

export const events = __makeEvents__<{
gitCommandEvent: GitCommandEvent
}>({
gitCommandEvent: "git-command-event"
})

/** user-defined types **/

export type Branches = { local: LocalBranch[]; remote: RemoteBranch[] }
export type GitCommandEvent = string
export type GitHash = string
export type LocalBranch = { head: boolean; 
/**
 * e.g. `["feat", "implement-stuff"]`
 */
name: string[]; upstream_name: string[]; upstream_track: UpstreamTrack; hash: GitHash }
export type RemoteBranch = { 
/**
 * e.g. `["origin", "feat", "implement-stuff"]`
 */
name: string[]; hash: GitHash }
export type Repository = { id: number; name: string; local_path: string; created_at: string; last_opened_at: string | null; last_fetched_at: string | null; has_changes: boolean }
export type State = { id: number; open_repository: number | null }
/**
 * If both are 0, it's in sync. If None, the tracked upstream is missing.
 */
export type UpstreamTrack = [number, number] | null

/** tauri-specta globals **/

         import { invoke as TAURI_INVOKE } from "@tauri-apps/api/core";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindow as __WebviewWindow__ } from "@tauri-apps/api/webviewWindow";

type __EventObj__<T> = {
  listen: (
    cb: TAURI_API_EVENT.EventCallback<T>
  ) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
  once: (
    cb: TAURI_API_EVENT.EventCallback<T>
  ) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
  emit: T extends null
    ? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
    : (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

export type Result<T, E> =
  | { status: "ok"; data: T }
  | { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
  mappings: Record<keyof T, string>
) {
  return new Proxy(
    {} as unknown as {
      [K in keyof T]: __EventObj__<T[K]> & {
        (handle: __WebviewWindow__): __EventObj__<T[K]>;
      };
    },
    {
      get: (_, event) => {
        const name = mappings[event as keyof T];

        return new Proxy((() => {}) as any, {
          apply: (_, __, [window]: [__WebviewWindow__]) => ({
            listen: (arg: any) => window.listen(name, arg),
            once: (arg: any) => window.once(name, arg),
            emit: (arg: any) => window.emit(name, arg),
          }),
          get: (_, command: keyof __EventObj__<any>) => {
            switch (command) {
              case "listen":
                return (arg: any) => TAURI_API_EVENT.listen(name, arg);
              case "once":
                return (arg: any) => TAURI_API_EVENT.once(name, arg);
              case "emit":
                return (arg: any) => TAURI_API_EVENT.emit(name, arg);
            }
          },
        });
      },
    }
  );
}

     