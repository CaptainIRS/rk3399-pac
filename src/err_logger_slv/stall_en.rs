#[doc = "Register `StallEn` reader"]
pub type R = crate::R<StallEnSpec>;
#[doc = "Register `StallEn` writer"]
pub type W = crate::W<StallEnSpec>;
#[doc = "Field `STALLEN` reader - When set to 1, enables stall mode. When set to 0, only the first\n\nerror packet is stored in the error logger."]
pub type StallenR = crate::BitReader;
#[doc = "Field `STALLEN` writer - When set to 1, enables stall mode. When set to 0, only the first\n\nerror packet is stored in the error logger."]
pub type StallenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set to 1, enables stall mode. When set to 0, only the first\n\nerror packet is stored in the error logger."]
    #[inline(always)]
    pub fn stallen(&self) -> StallenR {
        StallenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set to 1, enables stall mode. When set to 0, only the first\n\nerror packet is stored in the error logger."]
    #[inline(always)]
    #[must_use]
    pub fn stallen(&mut self) -> StallenW<StallEnSpec> {
        StallenW::new(self, 0)
    }
}
#[doc = "Error logger mode selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stall_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stall_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StallEnSpec;
impl crate::RegisterSpec for StallEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stall_en::R`](R) reader structure"]
impl crate::Readable for StallEnSpec {}
#[doc = "`write(|w| ..)` method takes [`stall_en::W`](W) writer structure"]
impl crate::Writable for StallEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets StallEn to value 0"]
impl crate::Resettable for StallEnSpec {
    const RESET_VALUE: u32 = 0;
}
