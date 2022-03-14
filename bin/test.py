import threading
import time
start = time.time()
people = 500      # 假设有500个人
def action(num):
    global people
    while people>0:
        people -= 50     # 每次运输50人
        print("车辆编号：%d, 当前车站人数：%d" %(num, people))

num = 1     # 车辆编号
vehicle = threading.Thread(target=action, args=(num,))  # 新建车辆
vehicle.start()     # 启动车辆
vehicle.join()      # 检查到站车辆

end = time.time()
print("Duration time: %0.3f" %(end-start))