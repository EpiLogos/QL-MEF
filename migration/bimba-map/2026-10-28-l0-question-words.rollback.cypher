// Rollback of 2026-10-28-l0-question-words.cypher: restores the six values exactly as read
// before the correction (map read 28 Oct 2026).

// @apply restore
MATCH (n:Bimba {coordinate:'L0-3'}) WHERE n.c_1_name = 'Who / Which / Whereby' SET n.c_1_name = 'Whom / Which / When';
MATCH (n:Bimba {coordinate:'L0-3'}) WHERE n.l_0_position_interpretation = 'Who/which/whereby reads P3 as pattern and determining form.' SET n.l_0_position_interpretation = 'Who/which/when reads P3 as pattern and determining form.';
MATCH (n:Bimba {coordinate:'L0-4'}) WHERE n.c_1_name = 'Where / When / Whither' SET n.c_1_name = 'Where / When / Why-four';
MATCH (n:Bimba {coordinate:'L0-4'}) WHERE n.l_0_position_interpretation = 'Where/when/whither reads P4 as context and horizon.' SET n.l_0_position_interpretation = 'Where/when/why-four reads P4 as context and horizon.';
MATCH (n:Bimba {coordinate:'L0-5'}) WHERE n.c_1_name = 'Why-for / Why-not' SET n.c_1_name = 'Why-so / Why-not';
MATCH (n:Bimba {coordinate:'L0-5'}) WHERE n.l_0_position_interpretation = 'Why-for/why-not reads P5 as synthesis and return.' SET n.l_0_position_interpretation = 'Why-so/why-not reads P5 as synthesis and return.';

// @report
MATCH (n:Bimba) WHERE n.coordinate IN ['L0-3','L0-4','L0-5']
RETURN n.coordinate, n.c_1_name, n.l_0_position_interpretation ORDER BY n.coordinate;
