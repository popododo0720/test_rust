fn verify(schedule: usize, timelog: usize) -> usize {
    let target_hour: usize = schedule/100;
    let target_min: usize = schedule%100;

    let now_hour: usize = timelog/100;
    let now_min: usize = timelog%100;

    if target_hour > now_hour {
        0
    } else if target_hour == now_hour {
        if target_min + 10 >= now_min { 0 } else { 1 }
    } else {
        1
    }
}

fn solution(schedules: Vec<usize> , timelogs: Vec<Vec<usize>>, startday: usize) -> usize{
    let mut week: Vec<usize> = Vec::new();
    let mut date: usize;

    for day_index in 0..7 {
        if startday + day_index > 7 { date = startday + day_index - 7 }
        else { date = startday + day_index };
        week.push(date);
    }

    let mut result: usize = 0;
    for schedules_index in 0..schedules.len() {
        let mut flag: usize = 0;
        let now_target_time: usize = schedules[schedules_index];

        for timelogs_date in 0..7 {
            let now_date: usize = week[timelogs_date];
            if now_date == 6 || now_date == 7 {
                continue;
            } 
            
            flag += verify(now_target_time, timelogs[schedules_index][timelogs_date]);
        }
        
        if flag == 0 {
            result += 1;
        }
    }
    
    result
}

fn main() {
    // test case 1
    // let schedules: Vec<usize> = vec![700, 800, 1100]; 
    // let timelogs: Vec<Vec<usize>> = vec![
    //     vec![710, 2359, 1050, 700, 650, 631, 659],  
    //     vec![800, 801, 805, 800, 759, 810, 809], 
    //     vec![1105, 1001, 1002, 600, 1059, 1001, 1100],  
    // ];
    // let startday: usize = 5;

    // test case 2
    let schedules: Vec<usize> = vec![730, 855, 700, 720]; 
    let timelogs: Vec<Vec<usize>> = vec![
        vec![710, 700, 650, 735, 700, 931, 912],  
        vec![908, 901, 805, 815, 800, 831, 835], 
        vec![705, 701, 702, 705, 710, 710, 711],  
        vec![707, 731, 859, 913, 934, 931, 905],  
    ];
    let startday: usize = 1;

    println!("{}", solution(schedules, timelogs, startday));
}
