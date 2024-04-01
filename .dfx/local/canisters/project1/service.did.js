export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'draw_winner' : IDL.Func([], [IDL.Opt(IDL.Principal)], []),
    'is_active' : IDL.Func([], [IDL.Bool], []),
    'participate' : IDL.Func([], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
