#[doc = "Register `Counters_0_Val` reader"]
pub type R = crate::R<Counters0ValSpec>;
#[doc = "Field `COUNTERS_0_VAL` reader - Register Val is a read-only register that is always present. The\n\nregister containsthe statistics counter value either pending\n\nStatAlarm output, or when statisticscollection is suspended\n\nsubsequent to triggers or signal statSuspend."]
pub type Counters0ValR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Register Val is a read-only register that is always present. The\n\nregister containsthe statistics counter value either pending\n\nStatAlarm output, or when statisticscollection is suspended\n\nsubsequent to triggers or signal statSuspend."]
    #[inline(always)]
    pub fn counters_0_val(&self) -> Counters0ValR {
        Counters0ValR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Registers Counters_M_Val contain the statistics counter values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`counters_0_val::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Counters0ValSpec;
impl crate::RegisterSpec for Counters0ValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`counters_0_val::R`](R) reader structure"]
impl crate::Readable for Counters0ValSpec {}
#[doc = "`reset()` method sets Counters_0_Val to value 0"]
impl crate::Resettable for Counters0ValSpec {
    const RESET_VALUE: u32 = 0;
}