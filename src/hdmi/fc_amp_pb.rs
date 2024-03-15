#[doc = "Register `FC_AMP_PB[%s]` reader"]
pub type R = crate::R<FcAmpPbSpec>;
#[doc = "Register `FC_AMP_PB[%s]` writer"]
pub type W = crate::W<FcAmpPbSpec>;
#[doc = "Field `FC_AMP_PB` reader - Frame Composer AMP Packet Body Register Array"]
pub type FcAmpPbR = crate::FieldReader;
#[doc = "Field `FC_AMP_PB` writer - Frame Composer AMP Packet Body Register Array"]
pub type FcAmpPbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer AMP Packet Body Register Array"]
    #[inline(always)]
    pub fn fc_amp_pb(&self) -> FcAmpPbR {
        FcAmpPbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer AMP Packet Body Register Array"]
    #[inline(always)]
    #[must_use]
    pub fn fc_amp_pb(&mut self) -> FcAmpPbW<FcAmpPbSpec> {
        FcAmpPbW::new(self, 0)
    }
}
#[doc = "Frame Composer AMP Packet Body Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_amp_pb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_amp_pb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAmpPbSpec;
impl crate::RegisterSpec for FcAmpPbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_amp_pb::R`](R) reader structure"]
impl crate::Readable for FcAmpPbSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_amp_pb::W`](W) writer structure"]
impl crate::Writable for FcAmpPbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AMP_PB[%s]
to value 0"]
impl crate::Resettable for FcAmpPbSpec {
    const RESET_VALUE: u8 = 0;
}
