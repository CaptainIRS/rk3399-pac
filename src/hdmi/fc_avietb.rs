#[doc = "Register `FC_AVIETB[%s]` reader"]
pub type R = crate::R<FcAvietbSpec>;
#[doc = "Register `FC_AVIETB[%s]` writer"]
pub type W = crate::W<FcAvietbSpec>;
#[doc = "Field `FC_AVIETB` reader - Defines the AVI InfoFrame End of Top Bar value.\n\nFor more information, refer to the CEA-861-E\n\nspecification."]
pub type FcAvietbR = crate::FieldReader;
#[doc = "Field `FC_AVIETB` writer - Defines the AVI InfoFrame End of Top Bar value.\n\nFor more information, refer to the CEA-861-E\n\nspecification."]
pub type FcAvietbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Defines the AVI InfoFrame End of Top Bar value.\n\nFor more information, refer to the CEA-861-E\n\nspecification."]
    #[inline(always)]
    pub fn fc_avietb(&self) -> FcAvietbR {
        FcAvietbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Defines the AVI InfoFrame End of Top Bar value.\n\nFor more information, refer to the CEA-861-E\n\nspecification."]
    #[inline(always)]
    #[must_use]
    pub fn fc_avietb(&mut self) -> FcAvietbW<FcAvietbSpec> {
        FcAvietbW::new(self, 0)
    }
}
#[doc = "Frame Composer AVI Packet End of Top Bar Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_avietb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_avietb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAvietbSpec;
impl crate::RegisterSpec for FcAvietbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_avietb::R`](R) reader structure"]
impl crate::Readable for FcAvietbSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_avietb::W`](W) writer structure"]
impl crate::Writable for FcAvietbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AVIETB[%s]
to value 0"]
impl crate::Resettable for FcAvietbSpec {
    const RESET_VALUE: u8 = 0;
}
