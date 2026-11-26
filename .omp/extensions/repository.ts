// spec: docs/repo/agent-runtimes.md
import { register } from '../../.agents/hooks/pi.ts';
export default (api: Parameters<typeof register>[0]) => register(api, 'omp');
