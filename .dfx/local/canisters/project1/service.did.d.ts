import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'draw_winner' : ActorMethod<[], [] | [Principal]>,
  'is_active' : ActorMethod<[], boolean>,
  'participate' : ActorMethod<[], undefined>,
}
