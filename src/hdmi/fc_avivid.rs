#[doc = "Register `FC_AVIVID` reader"]
pub type R = crate::R<FcAvividSpec>;
#[doc = "Register `FC_AVIVID` writer"]
pub type W = crate::W<FcAvividSpec>;
#[doc = "Field `FC_AVIVID` reader - Configures the AVI InfoFrame Video Identification\n\ncode. For more information, refer to the CEA-861-\n\nE specification."]
pub type FcAvividR = crate::FieldReader;
#[doc = "Field `FC_AVIVID` writer - Configures the AVI InfoFrame Video Identification\n\ncode. For more information, refer to the CEA-861-\n\nE specification."]
pub type FcAvividW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FC_AVIVID_7` reader - Bit 7 of fc_avivid register"]
pub type FcAvivid7R = crate::BitReader;
#[doc = "Field `FC_AVIVID_7` writer - Bit 7 of fc_avivid register"]
pub type FcAvivid7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Configures the AVI InfoFrame Video Identification\n\ncode. For more information, refer to the CEA-861-\n\nE specification."]
    #[inline(always)]
    pub fn fc_avivid(&self) -> FcAvividR {
        FcAvividR::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Bit 7 of fc_avivid register"]
    #[inline(always)]
    pub fn fc_avivid_7(&self) -> FcAvivid7R {
        FcAvivid7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Configures the AVI InfoFrame Video Identification\n\ncode. For more information, refer to the CEA-861-\n\nE specification."]
    #[inline(always)]
    #[must_use]
    pub fn fc_avivid(&mut self) -> FcAvividW<FcAvividSpec> {
        FcAvividW::new(self, 0)
    }
    #[doc = "Bit 7 - Bit 7 of fc_avivid register"]
    #[inline(always)]
    #[must_use]
    pub fn fc_avivid_7(&mut self) -> FcAvivid7W<FcAvividSpec> {
        FcAvivid7W::new(self, 7)
    }
}
#[doc = "Frame Composer AVI Packet VIC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_avivid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_avivid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAvividSpec;
impl crate::RegisterSpec for FcAvividSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_avivid::R`](R) reader structure"]
impl crate::Readable for FcAvividSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_avivid::W`](W) writer structure"]
impl crate::Writable for FcAvividSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AVIVID to value 0"]
impl crate::Resettable for FcAvividSpec {
    const RESET_VALUE: u8 = 0;
}
