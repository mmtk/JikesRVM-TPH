/*
 *  This file is part of the Jikes RVM project (http://jikesrvm.org).
 *
 *  This file is licensed to You under the Common Public License (CPL);
 *  You may not use this file except in compliance with the License. You
 *  may obtain a copy of the License at
 *
 *      http://www.opensource.org/licenses/cpl1.0.php
 *
 *  See the COPYRIGHT.txt file distributed with this work for information
 *  regarding copyright ownership.
 */
package org.jikesrvm.adaptive;

import org.jikesrvm.classloader.RVMField;
import org.jikesrvm.classloader.VM_NormalMethod;
import static org.jikesrvm.runtime.VM_EntrypointHelper.getField;
import static org.jikesrvm.runtime.VM_EntrypointHelper.getMethod;

/**
 * Entrypoints that are valid when the build includes the adaptive optimization system.
 */
public interface VM_AosEntrypoints {
  VM_NormalMethod osrGetRefAtMethod =
      getMethod("Lorg/jikesrvm/osr/OSR_ObjectHolder;", "getRefAt", "(II)Ljava/lang/Object;");
  VM_NormalMethod osrCleanRefsMethod = getMethod("Lorg/jikesrvm/osr/OSR_ObjectHolder;", "cleanRefs", "(I)V");
  RVMField methodListenerNumSamplesField =
      getField("Lorg/jikesrvm/adaptive/measurements/listeners/VM_MethodListener;", "numSamples", "I");
  RVMField edgeListenerUpdateCalledField =
      getField("Lorg/jikesrvm/adaptive/measurements/listeners/VM_EdgeListener;", "updateCalled", "I");
  RVMField edgeListenerSamplesTakenField =
      getField("Lorg/jikesrvm/adaptive/measurements/listeners/VM_EdgeListener;", "samplesTaken", "I");
  RVMField yieldCountListenerNumYieldsField =
      getField("Lorg/jikesrvm/adaptive/measurements/listeners/VM_YieldCounterListener;", "numYields", "I");
  RVMField counterArrayManagerCounterArraysField =
      getField("Lorg/jikesrvm/adaptive/measurements/instrumentation/VM_CounterArrayManager;", "counterArrays", "[[D");
  RVMField invocationCountsField =
      getField("Lorg/jikesrvm/adaptive/recompilation/VM_InvocationCounts;", "counts", "[I");
  VM_NormalMethod invocationCounterTrippedMethod =
      getMethod("Lorg/jikesrvm/adaptive/recompilation/VM_InvocationCounts;", "counterTripped", "(I)V");
  RVMField globalCBSField =
      getField("Lorg/jikesrvm/adaptive/recompilation/instrumentation/VM_CounterBasedSampling;", "globalCounter", "I");
  RVMField processorCBSField = getField("Lorg/jikesrvm/scheduler/VM_Processor;", "processor_cbs_counter", "I");
  RVMField cbsResetValueField =
      getField("Lorg/jikesrvm/adaptive/recompilation/instrumentation/VM_CounterBasedSampling;", "resetValue", "I");
  RVMField specializedMethodsField =
      getField("Lorg/jikesrvm/compilers/opt/SpecializedMethodPool;",
               "specializedMethods",
               "[Lorg/jikesrvm/ArchitectureSpecific$CodeArray;");
}
