#[doc = "Register `GRF_CPU_STATUS0` reader"]
pub type R = crate::R<GrfCpuStatus0Spec>;
#[doc = "Register `GRF_CPU_STATUS0` writer"]
pub type W = crate::W<GrfCpuStatus0Spec>;
#[doc = "Field `WRMEMATTR_PD_CORE_L` reader - wrmemattr of pd_core_l status"]
pub type WrmemattrPdCoreLR = crate::FieldReader;
#[doc = "Field `WRMEMATTR_PD_CORE_L` writer - wrmemattr of pd_core_l status"]
pub type WrmemattrPdCoreLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RDMEMATTR_PD_CORE_L` reader - rdmemattr of pd_core_l status"]
pub type RdmemattrPdCoreLR = crate::FieldReader;
#[doc = "Field `RDMEMATTR_PD_CORE_L` writer - rdmemattr of pd_core_l status"]
pub type RdmemattrPdCoreLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RDMEMATTR_PD_CORE_B` reader - rdmemattr of pd_core_b status"]
pub type RdmemattrPdCoreBR = crate::FieldReader;
#[doc = "Field `RDMEMATTR_PD_CORE_B` writer - rdmemattr of pd_core_b status"]
pub type RdmemattrPdCoreBW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRMEMATTR_PD_CORE_B` reader - wrmemattr of pd_core_b status"]
pub type WrmemattrPdCoreBR = crate::FieldReader;
#[doc = "Field `WRMEMATTR_PD_CORE_B` writer - wrmemattr of pd_core_b status"]
pub type WrmemattrPdCoreBW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - wrmemattr of pd_core_l status"]
    #[inline(always)]
    pub fn wrmemattr_pd_core_l(&self) -> WrmemattrPdCoreLR {
        WrmemattrPdCoreLR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - rdmemattr of pd_core_l status"]
    #[inline(always)]
    pub fn rdmemattr_pd_core_l(&self) -> RdmemattrPdCoreLR {
        RdmemattrPdCoreLR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - rdmemattr of pd_core_b status"]
    #[inline(always)]
    pub fn rdmemattr_pd_core_b(&self) -> RdmemattrPdCoreBR {
        RdmemattrPdCoreBR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - wrmemattr of pd_core_b status"]
    #[inline(always)]
    pub fn wrmemattr_pd_core_b(&self) -> WrmemattrPdCoreBR {
        WrmemattrPdCoreBR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - wrmemattr of pd_core_l status"]
    #[inline(always)]
    #[must_use]
    pub fn wrmemattr_pd_core_l(&mut self) -> WrmemattrPdCoreLW<GrfCpuStatus0Spec> {
        WrmemattrPdCoreLW::new(self, 0)
    }
    #[doc = "Bits 8:15 - rdmemattr of pd_core_l status"]
    #[inline(always)]
    #[must_use]
    pub fn rdmemattr_pd_core_l(&mut self) -> RdmemattrPdCoreLW<GrfCpuStatus0Spec> {
        RdmemattrPdCoreLW::new(self, 8)
    }
    #[doc = "Bits 16:23 - rdmemattr of pd_core_b status"]
    #[inline(always)]
    #[must_use]
    pub fn rdmemattr_pd_core_b(&mut self) -> RdmemattrPdCoreBW<GrfCpuStatus0Spec> {
        RdmemattrPdCoreBW::new(self, 16)
    }
    #[doc = "Bits 24:31 - wrmemattr of pd_core_b status"]
    #[inline(always)]
    #[must_use]
    pub fn wrmemattr_pd_core_b(&mut self) -> WrmemattrPdCoreBW<GrfCpuStatus0Spec> {
        WrmemattrPdCoreBW::new(self, 24)
    }
}
#[doc = "cpu status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_cpu_status0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_cpu_status0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfCpuStatus0Spec;
impl crate::RegisterSpec for GrfCpuStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_cpu_status0::R`](R) reader structure"]
impl crate::Readable for GrfCpuStatus0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_cpu_status0::W`](W) writer structure"]
impl crate::Writable for GrfCpuStatus0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_CPU_STATUS0 to value 0"]
impl crate::Resettable for GrfCpuStatus0Spec {
    const RESET_VALUE: u32 = 0;
}
