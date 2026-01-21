using UnityEngine;

public class FoodObject : CellObject
{
    public int AmountGranted = 10;
    public override void PlayerEntered()
    {
        // TODO : D�truire l'objet nourriture 
        Destroy(this.gameObject);

        // TODO :  Appeler GameManager instance pour incrementer le compter de nourriture du joueur
        GameManager.Instance.ChangeFood(AmountGranted);

        Debug.Log("Food increased");
    }
}
