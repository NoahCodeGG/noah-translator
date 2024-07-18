import { useCallback, useRef, useState } from 'react'

export const useGetState = (initState: string | number | boolean | null) => {
  const [state, setState] = useState<string | number | boolean | null>(initState)
  const stateRef = useRef(state)
  stateRef.current = state
  const getState = useCallback(() => stateRef.current, [])
  return [state, setState, getState] as const
}
