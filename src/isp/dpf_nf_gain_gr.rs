#[doc = "Register `DPF_NF_GAIN_GR` reader"]
pub type R = crate::R<DpfNfGainGrSpec>;
#[doc = "Register `DPF_NF_GAIN_GR` writer"]
pub type W = crate::W<DpfNfGainGrSpec>;
#[doc = "Field `DPF_NF_GAIN_GR` reader - Noise Function (NF) Gain that replaces the AWB gain\n\nfor green pixels in a red line.\n\n12 bit unsigned integer format: gain=1 -> 0x100\n\n"]
pub type DpfNfGainGrR = crate::FieldReader<u16>;
#[doc = "Field `DPF_NF_GAIN_GR` writer - Noise Function (NF) Gain that replaces the AWB gain\n\nfor green pixels in a red line.\n\n12 bit unsigned integer format: gain=1 -> 0x100\n\n"]
pub type DpfNfGainGrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Noise Function (NF) Gain that replaces the AWB gain\n\nfor green pixels in a red line.\n\n12 bit unsigned integer format: gain=1 -> 0x100\n\n"]
    #[inline(always)]
    pub fn dpf_nf_gain_gr(&self) -> DpfNfGainGrR {
        DpfNfGainGrR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Noise Function (NF) Gain that replaces the AWB gain\n\nfor green pixels in a red line.\n\n12 bit unsigned integer format: gain=1 -> 0x100\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn dpf_nf_gain_gr(&mut self) -> DpfNfGainGrW<DpfNfGainGrSpec> {
        DpfNfGainGrW::new(self, 0)
    }
}
#[doc = "noise function gain for green in red pixels\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_nf_gain_gr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_nf_gain_gr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpfNfGainGrSpec;
impl crate::RegisterSpec for DpfNfGainGrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpf_nf_gain_gr::R`](R) reader structure"]
impl crate::Readable for DpfNfGainGrSpec {}
#[doc = "`write(|w| ..)` method takes [`dpf_nf_gain_gr::W`](W) writer structure"]
impl crate::Writable for DpfNfGainGrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPF_NF_GAIN_GR to value 0x0100"]
impl crate::Resettable for DpfNfGainGrSpec {
    const RESET_VALUE: u32 = 0x0100;
}
