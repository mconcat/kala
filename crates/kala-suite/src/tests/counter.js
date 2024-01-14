export default function Counter(state) {
    const count = state.ref(0)

    return {
        increment: () => count.value += 1,
        decrement: () => count.value -= 1,
        count: () => count.value,
    }
}