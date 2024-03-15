#[doc = "Register `GRF_CPU_STATUS5` reader"]
pub type R = crate::R<GrfCpuStatus5Spec>;
#[doc = "Register `GRF_CPU_STATUS5` writer"]
pub type W = crate::W<GrfCpuStatus5Spec>;
#[doc = "Field `CCI_NEVNTCNTOVERFLOW` reader - cci_nevntcntoverflow status bit"]
pub type CciNevntcntoverflowR = crate::FieldReader;
#[doc = "Field `CCI_NEVNTCNTOVERFLOW` writer - cci_nevntcntoverflow status bit"]
pub type CciNevntcntoverflowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIC_ECC_FATAL` reader - gic_ecc_fatal status bit"]
pub type GicEccFatalR = crate::BitReader;
#[doc = "Field `GIC_ECC_FATAL` writer - gic_ecc_fatal status bit"]
pub type GicEccFatalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIC_AXIM_ERR` reader - gic_axim_err status bit"]
pub type GicAximErrR = crate::BitReader;
#[doc = "Field `GIC_AXIM_ERR` writer - gic_axim_err status bit"]
pub type GicAximErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - cci_nevntcntoverflow status bit"]
    #[inline(always)]
    pub fn cci_nevntcntoverflow(&self) -> CciNevntcntoverflowR {
        CciNevntcntoverflowR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - gic_ecc_fatal status bit"]
    #[inline(always)]
    pub fn gic_ecc_fatal(&self) -> GicEccFatalR {
        GicEccFatalR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - gic_axim_err status bit"]
    #[inline(always)]
    pub fn gic_axim_err(&self) -> GicAximErrR {
        GicAximErrR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - cci_nevntcntoverflow status bit"]
    #[inline(always)]
    #[must_use]
    pub fn cci_nevntcntoverflow(&mut self) -> CciNevntcntoverflowW<GrfCpuStatus5Spec> {
        CciNevntcntoverflowW::new(self, 0)
    }
    #[doc = "Bit 8 - gic_ecc_fatal status bit"]
    #[inline(always)]
    #[must_use]
    pub fn gic_ecc_fatal(&mut self) -> GicEccFatalW<GrfCpuStatus5Spec> {
        GicEccFatalW::new(self, 8)
    }
    #[doc = "Bit 9 - gic_axim_err status bit"]
    #[inline(always)]
    #[must_use]
    pub fn gic_axim_err(&mut self) -> GicAximErrW<GrfCpuStatus5Spec> {
        GicAximErrW::new(self, 9)
    }
}
#[doc = "cpu status register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_cpu_status5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_cpu_status5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfCpuStatus5Spec;
impl crate::RegisterSpec for GrfCpuStatus5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_cpu_status5::R`](R) reader structure"]
impl crate::Readable for GrfCpuStatus5Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_cpu_status5::W`](W) writer structure"]
impl crate::Writable for GrfCpuStatus5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_CPU_STATUS5 to value 0"]
impl crate::Resettable for GrfCpuStatus5Spec {
    const RESET_VALUE: u32 = 0;
}
