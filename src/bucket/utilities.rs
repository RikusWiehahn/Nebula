//
//    ##    ####   ####  ###### #####  #####     ####  #   #  ####  #      ######  ####
//   #  #  #    # #    # #      #    #   #      #    #  # #  #    # #      #      #
//  #    # #      #      #####  #    #   #      #        #   #      #      #####   ####
//  ###### #      #      #      #####    #      #        #   #      #      #           #
//  #    # #    # #    # #      #        #      #    #   #   #    # #      #      #    #
//  #    #  ####   ####  ###### #        #       ####    #    ####  ###### ######  ####

pub fn accept_cycles() -> u64 {
    let cycles_available = ic_cdk::api::call::msg_cycles_available();
    let cycles_accepted = ic_cdk::api::call::msg_cycles_accept(cycles_available);

    return cycles_accepted;
}