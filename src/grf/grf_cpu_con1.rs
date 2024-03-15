#[doc = "Register `GRF_CPU_CON1` reader"]
pub type R = crate::R<GrfCpuCon1Spec>;
#[doc = "Register `GRF_CPU_CON1` writer"]
pub type W = crate::W<GrfCpuCon1Spec>;
#[doc = "Field `AWQOS_PD_CORE_L` reader - pd_core_l awqos bit control"]
pub type AwqosPdCoreLR = crate::FieldReader;
#[doc = "Field `AWQOS_PD_CORE_L` writer - pd_core_l awqos bit control"]
pub type AwqosPdCoreLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ARQOS_PD_CORE_L` reader - pd_core_l arqos bit control"]
pub type ArqosPdCoreLR = crate::FieldReader;
#[doc = "Field `ARQOS_PD_CORE_L` writer - pd_core_l arqos bit control"]
pub type ArqosPdCoreLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLUSTERIDAFF1_PD_CORE_L` reader - pd_core_l clusteridaff1 bit control"]
pub type Clusteridaff1PdCoreLR = crate::FieldReader;
#[doc = "Field `CLUSTERIDAFF1_PD_CORE_L` writer - pd_core_l clusteridaff1 bit control"]
pub type Clusteridaff1PdCoreLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GIC_ACTIVE_CORE_L` reader - pd_core_l gic_active bit control"]
pub type GicActiveCoreLR = crate::FieldReader;
#[doc = "Field `GIC_ACTIVE_CORE_L` writer - pd_core_l gic_active bit control"]
pub type GicActiveCoreLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - pd_core_l awqos bit control"]
    #[inline(always)]
    pub fn awqos_pd_core_l(&self) -> AwqosPdCoreLR {
        AwqosPdCoreLR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - pd_core_l arqos bit control"]
    #[inline(always)]
    pub fn arqos_pd_core_l(&self) -> ArqosPdCoreLR {
        ArqosPdCoreLR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - pd_core_l clusteridaff1 bit control"]
    #[inline(always)]
    pub fn clusteridaff1_pd_core_l(&self) -> Clusteridaff1PdCoreLR {
        Clusteridaff1PdCoreLR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - pd_core_l gic_active bit control"]
    #[inline(always)]
    pub fn gic_active_core_l(&self) -> GicActiveCoreLR {
        GicActiveCoreLR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - pd_core_l awqos bit control"]
    #[inline(always)]
    #[must_use]
    pub fn awqos_pd_core_l(&mut self) -> AwqosPdCoreLW<GrfCpuCon1Spec> {
        AwqosPdCoreLW::new(self, 0)
    }
    #[doc = "Bits 4:7 - pd_core_l arqos bit control"]
    #[inline(always)]
    #[must_use]
    pub fn arqos_pd_core_l(&mut self) -> ArqosPdCoreLW<GrfCpuCon1Spec> {
        ArqosPdCoreLW::new(self, 4)
    }
    #[doc = "Bits 8:11 - pd_core_l clusteridaff1 bit control"]
    #[inline(always)]
    #[must_use]
    pub fn clusteridaff1_pd_core_l(&mut self) -> Clusteridaff1PdCoreLW<GrfCpuCon1Spec> {
        Clusteridaff1PdCoreLW::new(self, 8)
    }
    #[doc = "Bits 12:15 - pd_core_l gic_active bit control"]
    #[inline(always)]
    #[must_use]
    pub fn gic_active_core_l(&mut self) -> GicActiveCoreLW<GrfCpuCon1Spec> {
        GicActiveCoreLW::new(self, 12)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfCpuCon1Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "cpu control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_cpu_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_cpu_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfCpuCon1Spec;
impl crate::RegisterSpec for GrfCpuCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_cpu_con1::R`](R) reader structure"]
impl crate::Readable for GrfCpuCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_cpu_con1::W`](W) writer structure"]
impl crate::Writable for GrfCpuCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_CPU_CON1 to value 0xf000"]
impl crate::Resettable for GrfCpuCon1Spec {
    const RESET_VALUE: u32 = 0xf000;
}
