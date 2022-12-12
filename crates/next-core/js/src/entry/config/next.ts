import IPC, { Ipc } from "@vercel/turbopack-next/internal/ipc";
import loadConfig from "next/dist/server/config";
import { PHASE_DEVELOPMENT_SERVER } from "next/dist/shared/lib/constants";

type IpcIncomingMessage = {
  type: "loadNextConfig";
  path: string;
};

type IpcOutgoingMessage = {
  type: "javaScriptValue";
  data: number[];
};

const ipc = IPC as Ipc<IpcIncomingMessage, IpcOutgoingMessage>;

(async () => {
  while (true) {
    const msg = await ipc.recv();

    switch (msg.type) {
      case "loadNextConfig": {
        const nextConfig = await loadConfig(PHASE_DEVELOPMENT_SERVER, msg.path);
        // @ts-expect-error
        nextConfig.rewrites = await nextConfig.rewrites?.();
        // @ts-expect-error
        nextConfig.redirects = await nextConfig.redirects?.();
        if (nextConfig.configFile) {
          // watch and invalidate the config file
          await import(nextConfig.configFile);
        }
        await ipc.send({
          type: "javaScriptValue",
          data: Array.from(Buffer.from(JSON.stringify(nextConfig))),
        });
        break;
      }
      default: {
        console.error("unexpected message type", msg.type);
        process.exit(1);
      }
    }
  }
})().catch((err) => {
  ipc.sendError(err);
});
