struct Node
{
    int data;
    struct Node *next;
};

int main()
{
    struct Node *head_p = 0;
    struct Node *second_p = 0;
    struct Node *third_p = 0;

    struct Node *temp = 0;

    struct Node node_a = { 0, 0 };
    struct Node node_b = { 0, 0 };
    struct Node node_c = { 0, 0 };

    head_p = &node_a;
    second_p = &node_b;
    third_p = &node_c;

    head_p->data = 1;
    second_p->data = 2;
    third_p->data = 3;

    head_p->next = &node_b;
    second_p->next = &node_c;
    
    temp = head_p;

    while (temp != 0)
    {
        printf("%d -> ", temp->data);
        temp = temp->next;
    }

    return 0;
}