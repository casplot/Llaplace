fn main() {
    let guess: u32 = "42".parse().expect("not a number");

    println!("{}", guess); 
    //rustһ���ĸ��������ͣ����������㣬�������ַ�(ע�⣬emojiҲ�������ַ�������unicode����)
    //������С�����֣��޷���u��ͷ���з���i���������ִ���ռ�ö�������λ����
    //isize��usizeλ���ɼ�����ܹ�������64λ��64��Ĭ������i32
    //�������������ģʽ��ʾpanic,����ģʽ���ƣ�����u8ģ2^8������panic

    //����f32/64(��˫����)����IEEE-754��׼��Ĭ������Ϊ����
    let x = 2.0;
    let y: f32 = 3.0; 
    //�������ͣ�Ԫ�飨tuple��������
    //tuple�������Ͷ�ָ����ͬһ���ͣ����ȹ̶������������ɸı�
    let tup: (i32, f32, u8) = (500, 6.4, 1);
    println!("{},{},{}", tup.0, tup.1, tup.2);
    //��ȡtupleֵ
    let (x, y, z) = tup;
    println!("{},{},{}", x, y,z);
    //����Ҳ���Խ���ֵ��ͬ���ͣ�����Ԫ�����ͱ�����ͬ��������̶�
    //�����������stack����heap�����뱣֤�й̶�����Ԫ�أ����������
    //����vector���ڣ���������
    let a:[i32;5] = [1, 2, 3, 4, 5];
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov","Dec"];
    let first = months[0];
    let second = months[1];
    //������stack�Ϸ���ĵ�������ڴ棬�����������ʣ�������Χ������ͨ�������б���rust�����������Ӧ��ַ���ڴ棩
    
    //����������fn�ؼ��֣��������β���ʵ�Σ�����ǩ���б�������ÿ����������
    //�������������ɣ���ѡ���ɱ��ʽ������rust���ڱ��ʽ�����Ϊִ�ж�����ָ����ʽ����һ��ֵ
    //�������弴��䣬������ֵ���ʲ�������let��һ����丳�����
    another_function(s);
    //->���ź�������������ֵ���͵�����Ϊ������
    //rust�У�����ֵ�������������һ�����ʽ��ֵ
    //������ǰ���أ�Ҫ��return����ָ��ֵ
    //��������Ŀ���ʽд��
    //ע�⣬if else��֧�������ͱ�����ͬһ��������
    let num = if condition {0} else {1};
    //rust����ѭ����loop while for
    //loop��rust����ִ��ĳ�����ֱ����ͣ����break����֪��ʱ��ͣ
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter = 10{
            break counter* 2;
        }
    };
    println!("result is: {}", result);
    while number != 0 {
        println!("{}", number);

        number = number - 1;
    } 
    
}

fn another_function(x: i32) {
    println!("another one!value of x is {}", x);
}