#[doc = "Register `DPF_NLL_COEFF%s` reader"]
pub type R = crate::R<DpfNllCoeffSpec>;
#[doc = "Register `DPF_NLL_COEFF%s` writer"]
pub type W = crate::W<DpfNllCoeffSpec>;
#[doc = "Field `nll_coeff_n` reader - Noise Level Lookup Table Coefficient nll_coeff_n\n\n10 bit unsigned, value range 1/1024 to 1023/1024\n\n(*Default*)\n\n"]
pub type NllCoeffNR = crate::FieldReader<u16>;
#[doc = "Field `nll_coeff_n` writer - Noise Level Lookup Table Coefficient nll_coeff_n\n\n10 bit unsigned, value range 1/1024 to 1023/1024\n\n(*Default*)\n\n"]
pub type NllCoeffNW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Noise Level Lookup Table Coefficient nll_coeff_n\n\n10 bit unsigned, value range 1/1024 to 1023/1024\n\n(*Default*)\n\n"]
    #[inline(always)]
    pub fn nll_coeff_n(&self) -> NllCoeffNR {
        NllCoeffNR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Noise Level Lookup Table Coefficient nll_coeff_n\n\n10 bit unsigned, value range 1/1024 to 1023/1024\n\n(*Default*)\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn nll_coeff_n(&mut self) -> NllCoeffNW<DpfNllCoeffSpec> {
        NllCoeffNW::new(self, 0)
    }
}
#[doc = "Noise Level Lookup Coefficient n (n=0..16)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_nll_coeff::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_nll_coeff::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpfNllCoeffSpec;
impl crate::RegisterSpec for DpfNllCoeffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpf_nll_coeff::R`](R) reader structure"]
impl crate::Readable for DpfNllCoeffSpec {}
#[doc = "`write(|w| ..)` method takes [`dpf_nll_coeff::W`](W) writer structure"]
impl crate::Writable for DpfNllCoeffSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPF_NLL_COEFF%s to value 0x03ff"]
impl crate::Resettable for DpfNllCoeffSpec {
    const RESET_VALUE: u32 = 0x03ff;
}
