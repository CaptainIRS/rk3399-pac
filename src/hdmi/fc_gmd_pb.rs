#[doc = "Register `FC_GMD_PB[%s]` reader"]
pub type R = crate::R<FcGmdPbSpec>;
#[doc = "Register `FC_GMD_PB[%s]` writer"]
pub type W = crate::W<FcGmdPbSpec>;
#[doc = "Field `FC_GMD_PB` reader - Frame Composer GMD Packet Body Register Array"]
pub type FcGmdPbR = crate::FieldReader;
#[doc = "Field `FC_GMD_PB` writer - Frame Composer GMD Packet Body Register Array"]
pub type FcGmdPbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer GMD Packet Body Register Array"]
    #[inline(always)]
    pub fn fc_gmd_pb(&self) -> FcGmdPbR {
        FcGmdPbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer GMD Packet Body Register Array"]
    #[inline(always)]
    #[must_use]
    pub fn fc_gmd_pb(&mut self) -> FcGmdPbW<FcGmdPbSpec> {
        FcGmdPbW::new(self, 0)
    }
}
#[doc = "Frame Composer GMD Packet Body Register Array Configures the GMD packet\n\nbody of the GMD packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_gmd_pb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_gmd_pb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcGmdPbSpec;
impl crate::RegisterSpec for FcGmdPbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_gmd_pb::R`](R) reader structure"]
impl crate::Readable for FcGmdPbSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_gmd_pb::W`](W) writer structure"]
impl crate::Writable for FcGmdPbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_GMD_PB[%s]
to value 0"]
impl crate::Resettable for FcGmdPbSpec {
    const RESET_VALUE: u8 = 0;
}
