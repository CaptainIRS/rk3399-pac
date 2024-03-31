#[doc = "Register `STET` reader"]
pub type R = crate::R<StetSpec>;
#[doc = "Register `STET` writer"]
pub type W = crate::W<StetSpec>;
#[doc = "Field `SHADOW_TX_EMPTY_TRIGGER` reader - Shadow TX Empty Trigger.\n\nThis is a shadow register for the TX empty trigger bits\n\n(FCR\\[5:4\\])."]
pub type ShadowTxEmptyTriggerR = crate::BitReader;
#[doc = "Field `SHADOW_TX_EMPTY_TRIGGER` writer - Shadow TX Empty Trigger.\n\nThis is a shadow register for the TX empty trigger bits\n\n(FCR\\[5:4\\])."]
pub type ShadowTxEmptyTriggerW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Shadow TX Empty Trigger.\n\nThis is a shadow register for the TX empty trigger bits\n\n(FCR\\[5:4\\])."]
    #[inline(always)]
    pub fn shadow_tx_empty_trigger(&self) -> ShadowTxEmptyTriggerR {
        ShadowTxEmptyTriggerR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shadow TX Empty Trigger.\n\nThis is a shadow register for the TX empty trigger bits\n\n(FCR\\[5:4\\])."]
    #[inline(always)]
    #[must_use]
    pub fn shadow_tx_empty_trigger(&mut self) -> ShadowTxEmptyTriggerW<StetSpec> {
        ShadowTxEmptyTriggerW::new(self, 0)
    }
}
#[doc = "Shadow TX Empty Trigger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stet::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stet::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StetSpec;
impl crate::RegisterSpec for StetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stet::R`](R) reader structure"]
impl crate::Readable for StetSpec {}
#[doc = "`write(|w| ..)` method takes [`stet::W`](W) writer structure"]
impl crate::Writable for StetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STET to value 0"]
impl crate::Resettable for StetSpec {
    const RESET_VALUE: u32 = 0;
}
