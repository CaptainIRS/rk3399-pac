#[doc = "Register `FC_NVBI_PB[%s]` reader"]
pub type R = crate::R<FcNvbiPbSpec>;
#[doc = "Register `FC_NVBI_PB[%s]` writer"]
pub type W = crate::W<FcNvbiPbSpec>;
#[doc = "Field `FC_NVBI_PB` reader - Frame Composer NTSC VBI Packet Body Register\n\nArray"]
pub type FcNvbiPbR = crate::FieldReader;
#[doc = "Field `FC_NVBI_PB` writer - Frame Composer NTSC VBI Packet Body Register\n\nArray"]
pub type FcNvbiPbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer NTSC VBI Packet Body Register\n\nArray"]
    #[inline(always)]
    pub fn fc_nvbi_pb(&self) -> FcNvbiPbR {
        FcNvbiPbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer NTSC VBI Packet Body Register\n\nArray"]
    #[inline(always)]
    #[must_use]
    pub fn fc_nvbi_pb(&mut self) -> FcNvbiPbW<FcNvbiPbSpec> {
        FcNvbiPbW::new(self, 0)
    }
}
#[doc = "Frame Composer NTSC VBI Packet Body Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_nvbi_pb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_nvbi_pb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcNvbiPbSpec;
impl crate::RegisterSpec for FcNvbiPbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_nvbi_pb::R`](R) reader structure"]
impl crate::Readable for FcNvbiPbSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_nvbi_pb::W`](W) writer structure"]
impl crate::Writable for FcNvbiPbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_NVBI_PB[%s]
to value 0"]
impl crate::Resettable for FcNvbiPbSpec {
    const RESET_VALUE: u8 = 0;
}
