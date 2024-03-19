#[doc = "Register `FC_SPDDEVICEINF` reader"]
pub type R = crate::R<FcSpddeviceinfSpec>;
#[doc = "Register `FC_SPDDEVICEINF` writer"]
pub type W = crate::W<FcSpddeviceinfSpec>;
#[doc = "Field `FC_SPDDEVICEINF` reader - Frame Composer SPD Packet Data Source Product\n\nDescriptor Register"]
pub type FcSpddeviceinfR = crate::FieldReader;
#[doc = "Field `FC_SPDDEVICEINF` writer - Frame Composer SPD Packet Data Source Product\n\nDescriptor Register"]
pub type FcSpddeviceinfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer SPD Packet Data Source Product\n\nDescriptor Register"]
    #[inline(always)]
    pub fn fc_spddeviceinf(&self) -> FcSpddeviceinfR {
        FcSpddeviceinfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer SPD Packet Data Source Product\n\nDescriptor Register"]
    #[inline(always)]
    #[must_use]
    pub fn fc_spddeviceinf(&mut self) -> FcSpddeviceinfW<FcSpddeviceinfSpec> {
        FcSpddeviceinfW::new(self, 0)
    }
}
#[doc = "Frame Composer SPD Packet Data Source Product Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_spddeviceinf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_spddeviceinf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcSpddeviceinfSpec;
impl crate::RegisterSpec for FcSpddeviceinfSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_spddeviceinf::R`](R) reader structure"]
impl crate::Readable for FcSpddeviceinfSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_spddeviceinf::W`](W) writer structure"]
impl crate::Writable for FcSpddeviceinfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_SPDDEVICEINF to value 0"]
impl crate::Resettable for FcSpddeviceinfSpec {
    const RESET_VALUE: u8 = 0;
}
