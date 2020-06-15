import { useEffect, useMemo } from 'react';

export class Scheduler {
  private timer?: NodeJS.Timeout;

  public schedule(action: () => void, delay = 0) {
    this.cancel();
    this.timer = global.setTimeout(action, delay);
  }

  public cancel() {
    if (this.timer) {
      clearTimeout(this.timer);
      this.timer = undefined;
    }
  }

  public get isRunning() {
    return this.timer !== undefined;
  }
}

export function useScheduler() {
  const closeScheduler = useMemo(() => new Scheduler(), []);

  useEffect(() => {
    return () => closeScheduler.cancel();
  }, []);

  return closeScheduler;
}
