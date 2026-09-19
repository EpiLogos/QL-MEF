// Typed authoring specimen; native lowering and execution are supplied by VW1.
export type Source = Readonly<{ ref: string; revision: string }>;
export type CP = '4.0' | '4.1' | '4.2' | '4.3' | '4.4' | '4.5';
export type CF = 'CF1' | 'CF2' | 'CF3' | 'CF4' | 'CF5' | 'CF6' | 'CF7';
export type CFP = 'CFP0' | 'CFP1' | 'CFP2' | 'CFP3' | 'CFP4' | 'CFP5';
export type CS = 'CS0' | 'CS1' | 'CS2' | 'CS3' | 'CS4' | 'CS5';
export type CT = 'CT0' | 'CT1' | 'CT2' | 'CT3' | 'CT4' | 'CT4b′' | 'CT5';
export type Participation =
  | Readonly<{ CPF: 'dialogical' }>
  | Readonly<{ CPF: 'authorised-undertaking'; authority: Source }>;
export type CPrime = Participation & Readonly<{
  CT: CT; CP: CP; CF: CF; CFP: CFP; CS: CS;
  direction: 'forward' | 'returning';
  interpretation: Source;
  harmonicBasis: Source;
}>;

export const relations = {
  '@#': 'Open potential.',
  '-': 'Distinguish.',
  '+': 'Affirm or include.',
  x: 'Relate by, as, through or where.',
  '/': 'Hold in context and dialectical relation.',
  '=': 'Express a determination; naming retains its expansion.'
} as const;

export const horizons = {
  '@0': 'Available ground and knowledge.',
  '@1': 'Original determining structure.',
  '@2': 'Reflection and meaning.',
  '@3': 'Language, symbol and form.',
  '@4': 'World, context and story.',
  '@5': 'Power, instrument and praxis.'
} as const;

export const threadForms = {
  CFP0: 'One voice.',
  CFP1: 'Independent voices, composed as a chord.',
  CFP2: 'A returned result becomes the next voice’s material.',
  CFP3: 'Different readings meet in an independently examined fusion.',
  CFP4: 'Sustained work, with continuation and an explicit stop.',
  CFP5: 'A composition participates within another composition.'
} as const satisfies Record<CFP, string>;

export const passages = {
  CS0: [['4.0', '4.5'], ['4.1', '4.4'], ['4.2', '4.3'], ['4.3', '4.2'], ['4.4', '4.1'], ['4.5', '4.0']],
  CS1: [['4.0', '4.5'], ['4.1', '4.4']],
  CS2: [['4.0', '4.5'], ['4.1', '4.4'], ['4.2', '4.3']],
  CS3: [['4.0', '4.5'], ['4.1', '4.4'], ['4.2', '4.3'], ['4.3', '4.2']],
  CS4: [['4.0', '4.5'], ['4.4', '4.1'], ['4.5', '4.0']],
  CS5: [['4.0', '4.5'], ['4.5', '4.0']]
} as const satisfies Record<CS, readonly (readonly [CP, CP])[]>;

export const constitutionalRoles = {
  Nous: { CF: 'CF1', description: 'Open the concern and recover its ground before deciding its form.' },
  Logos: { CF: 'CF2', description: 'Articulate the distinctions, constraints and source-bearing form.' },
  Eros: { CF: 'CF3', description: 'Bring the present need into exchange with real available powers.' },
  Mythos: { CF: 'CF4', description: 'Compose the organising image without losing what it expresses.' },
  Anima: { CF: 'CF5', description: 'Compose and conduct the situated undertaking and its voices.' },
  Psyche: { CF: 'CF6', description: 'Steward continuity and the distribution of context and faculties.' },
  Sophia: { CF: 'CF7', description: 'Integrate the performed difference into renewed ground.' }
} as const satisfies Record<string, Readonly<{ CF: CF; description: string }>>;

export const specialists = {
  Anansi: 'Coordinate and blueprint work.',
  Janus: 'Temporal and threshold work.',
  Moirai: 'GraphRAG distillation through the relevant source-defined mode.',
  Mercurius: 'Kairos and qualitative temporal patterns.',
  Agora: 'Aggregation and skill/plugin absorption.',
  Zeithoven: 'Creative advance and skill/agent creation.'
} as const;

export type Role = keyof typeof constitutionalRoles | keyof typeof specialists;
export type Faculty =
  | 'search' | 'inspect' | 'compose' | 'perform' | 'observe' | 'interrupt'
  | 'save' | 'propose-practice' | 'return';

export interface Participant {
  readonly key: string;
  readonly role: Role;
  readonly roleSource: Source;
  readonly agent: Source;
  readonly description: string;
  readonly prompt: string;
  readonly context: readonly Source[];
  readonly skills: readonly Source[];
  readonly faculties: readonly Faculty[];
  readonly permittedEffects: readonly string[];
  readonly bodyRequirement: Source;
  readonly expectedReturn: string;
}

export interface Act {
  readonly key: string;
  readonly actor: string;
  readonly concern: string;
  readonly requiredDifference: string;
  readonly composition: CPrime;
  readonly capabilities: readonly Source[];
  readonly dependsOn: readonly string[];
  readonly independentFrom: readonly string[];
  readonly returnsFrom: readonly Readonly<{ act: string; selection: string }>[];
  readonly returnContract: string;
  readonly stop: string;
}

export interface Ground {
  readonly source: Source;
  readonly world: Source;
  readonly project: Source;
  readonly now: Source;
  readonly subject: Source;
  readonly whole: Source;
  readonly vakSource: Source;
  readonly nativeResolveExpression: Source;
  readonly initialExpression: Source;
  readonly returnAddress: string;
  readonly budget: Readonly<{ maxConcurrentActors: number; maxIterations: number }>;
}

export interface VakWorkflow {
  readonly title: string;
  readonly intention: string;
  readonly ground: Ground;
  readonly composition: CPrime;
  readonly participants: readonly Participant[];
  readonly acts: readonly Act[];
  readonly barriers: readonly Readonly<{
    key: string; waitsFor: readonly string[]; releases: readonly string[];
  }>[];
  readonly nesting: readonly Readonly<{ parent: string; child: string }>[];
  readonly returnCycle: readonly ['compose', 'perform', 'record', 'rehear', 'recompose'];
}

export interface CraftInputs {
  readonly ground: Ground;
  readonly composition: CPrime;
  readonly participants: readonly Participant[];
  readonly acts: readonly Act[];
  readonly barriers: VakWorkflow['barriers'];
  readonly nesting: VakWorkflow['nesting'];
}

export function expressionCraft(input: CraftInputs): VakWorkflow {
  return {
    title: 'A living expressive repertoire',
    intention: 'Make, inspect and refine a source-bound Expression; retain useful form for later variation and invocation.',
    ground: input.ground,
    composition: input.composition,
    participants: input.participants,
    acts: input.acts,
    barriers: input.barriers,
    nesting: input.nesting,
    returnCycle: ['compose', 'perform', 'record', 'rehear', 'recompose']
  };
}
