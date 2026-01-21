using UnityEngine;

public class TurnManager
{
    private int m_TurnCount;
    public event System.Action OnTick;

    public TurnManager()
    {
        m_TurnCount = 0;
    }

    public void Tick()
    {
        // TODO : incr�mente m_TurnCount
        m_TurnCount++;
        //Debug.Log("Turn: " + m_TurnCount);
        OnTick?.Invoke();
    }
}

