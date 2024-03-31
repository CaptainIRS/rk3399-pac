#[doc = "Register `CPU_CON3` reader"]
pub type R = crate::R<CpuCon3Spec>;
#[doc = "Register `CPU_CON3` writer"]
pub type W = crate::W<CpuCon3Spec>;
#[doc = "Field `AWQOS_PD_CORE_B` reader - pd_core_b awqos bit control"]
pub type AwqosPdCoreBR = crate::FieldReader;
#[doc = "Field `AWQOS_PD_CORE_B` writer - pd_core_b awqos bit control"]
pub type AwqosPdCoreBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ARQOS_PD_CORE_B` reader - pd_core_b arqos bit control"]
pub type ArqosPdCoreBR = crate::FieldReader;
#[doc = "Field `ARQOS_PD_CORE_B` writer - pd_core_b arqos bit control"]
pub type ArqosPdCoreBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLUSTERIDAFF1_PD_CORE_B` reader - pd_core_b clusteridaff1 bit control"]
pub type Clusteridaff1PdCoreBR = crate::FieldReader;
#[doc = "Field `CLUSTERIDAFF1_PD_CORE_B` writer - pd_core_b clusteridaff1 bit control"]
pub type Clusteridaff1PdCoreBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GIC_ACTIVE_CORE_B` reader - pd_core_b gic_active bit control"]
pub type GicActiveCoreBR = crate::FieldReader;
#[doc = "Field `GIC_ACTIVE_CORE_B` writer - pd_core_b gic_active bit control"]
pub type GicActiveCoreBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - pd_core_b awqos bit control"]
    #[inline(always)]
    pub fn awqos_pd_core_b(&self) -> AwqosPdCoreBR {
        AwqosPdCoreBR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - pd_core_b arqos bit control"]
    #[inline(always)]
    pub fn arqos_pd_core_b(&self) -> ArqosPdCoreBR {
        ArqosPdCoreBR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - pd_core_b clusteridaff1 bit control"]
    #[inline(always)]
    pub fn clusteridaff1_pd_core_b(&self) -> Clusteridaff1PdCoreBR {
        Clusteridaff1PdCoreBR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - pd_core_b gic_active bit control"]
    #[inline(always)]
    pub fn gic_active_core_b(&self) -> GicActiveCoreBR {
        GicActiveCoreBR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - pd_core_b awqos bit control"]
    #[inline(always)]
    #[must_use]
    pub fn awqos_pd_core_b(&mut self) -> AwqosPdCoreBW<CpuCon3Spec> {
        AwqosPdCoreBW::new(self, 0)
    }
    #[doc = "Bits 4:5 - pd_core_b arqos bit control"]
    #[inline(always)]
    #[must_use]
    pub fn arqos_pd_core_b(&mut self) -> ArqosPdCoreBW<CpuCon3Spec> {
        ArqosPdCoreBW::new(self, 4)
    }
    #[doc = "Bits 8:9 - pd_core_b clusteridaff1 bit control"]
    #[inline(always)]
    #[must_use]
    pub fn clusteridaff1_pd_core_b(&mut self) -> Clusteridaff1PdCoreBW<CpuCon3Spec> {
        Clusteridaff1PdCoreBW::new(self, 8)
    }
    #[doc = "Bits 12:13 - pd_core_b gic_active bit control"]
    #[inline(always)]
    #[must_use]
    pub fn gic_active_core_b(&mut self) -> GicActiveCoreBW<CpuCon3Spec> {
        GicActiveCoreBW::new(self, 12)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<CpuCon3Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "cpu control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_con3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_con3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuCon3Spec;
impl crate::RegisterSpec for CpuCon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_con3::R`](R) reader structure"]
impl crate::Readable for CpuCon3Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_con3::W`](W) writer structure"]
impl crate::Writable for CpuCon3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU_CON3 to value 0x3110"]
impl crate::Resettable for CpuCon3Spec {
    const RESET_VALUE: u32 = 0x3110;
}
