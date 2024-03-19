#[doc = "Register `EMMCCORE_SLOTINTSTS` reader"]
pub type R = crate::R<EmmccoreSlotintstsSpec>;
#[doc = "Field `INTSLOT0` reader - This status bit indicates the OR of Interrupt signal and Wakeup\n\nsignal for slot"]
pub type Intslot0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This status bit indicates the OR of Interrupt signal and Wakeup\n\nsignal for slot"]
    #[inline(always)]
    pub fn intslot0(&self) -> Intslot0R {
        Intslot0R::new((self.bits & 1) != 0)
    }
}
#[doc = "Slot interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_slotintsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreSlotintstsSpec;
impl crate::RegisterSpec for EmmccoreSlotintstsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`emmccore_slotintsts::R`](R) reader structure"]
impl crate::Readable for EmmccoreSlotintstsSpec {}
#[doc = "`reset()` method sets EMMCCORE_SLOTINTSTS to value 0"]
impl crate::Resettable for EmmccoreSlotintstsSpec {
    const RESET_VALUE: u16 = 0;
}
