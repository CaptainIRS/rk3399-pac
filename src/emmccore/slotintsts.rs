#[doc = "Register `SLOTINTSTS` reader"]
pub type R = crate::R<SlotintstsSpec>;
#[doc = "Field `INTSLOT0` reader - This status bit indicates the OR of Interrupt signal and Wakeup\n\nsignal for slot"]
pub type Intslot0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This status bit indicates the OR of Interrupt signal and Wakeup\n\nsignal for slot"]
    #[inline(always)]
    pub fn intslot0(&self) -> Intslot0R {
        Intslot0R::new((self.bits & 1) != 0)
    }
}
#[doc = "Slot interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slotintsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlotintstsSpec;
impl crate::RegisterSpec for SlotintstsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`slotintsts::R`](R) reader structure"]
impl crate::Readable for SlotintstsSpec {}
#[doc = "`reset()` method sets SLOTINTSTS to value 0"]
impl crate::Resettable for SlotintstsSpec {
    const RESET_VALUE: u16 = 0;
}
