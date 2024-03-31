#[doc = "Register `CPU_STATUS1` reader"]
pub type R = crate::R<CpuStatus1Spec>;
#[doc = "Register `CPU_STATUS1` writer"]
pub type W = crate::W<CpuStatus1Spec>;
#[doc = "Field `STANDBYWFI_PD_CORE_L` reader - standbywfi of pd_core_l status bit"]
pub type StandbywfiPdCoreLR = crate::FieldReader;
#[doc = "Field `STANDBYWFI_PD_CORE_L` writer - standbywfi of pd_core_l status bit"]
pub type StandbywfiPdCoreLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STANDBYWFE_PD_CORE_L` reader - standbywfe of pd_core_l status bit"]
pub type StandbywfePdCoreLR = crate::FieldReader;
#[doc = "Field `STANDBYWFE_PD_CORE_L` writer - standbywfe of pd_core_l status bit"]
pub type StandbywfePdCoreLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STANDBYWFI_PD_CORE_B` reader - standbywfi of pd_core_b status bit"]
pub type StandbywfiPdCoreBR = crate::FieldReader;
#[doc = "Field `STANDBYWFI_PD_CORE_B` writer - standbywfi of pd_core_b status bit"]
pub type StandbywfiPdCoreBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STANDBYWFE_PD_CORE_B` reader - standbywfe of pd_core_b status bit"]
pub type StandbywfePdCoreBR = crate::FieldReader;
#[doc = "Field `STANDBYWFE_PD_CORE_B` writer - standbywfe of pd_core_b status bit"]
pub type StandbywfePdCoreBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SMPEN_PD_CORE_L` reader - status of smpen_pd_core_l"]
pub type SmpenPdCoreLR = crate::FieldReader;
#[doc = "Field `SMPEN_PD_CORE_L` writer - status of smpen_pd_core_l"]
pub type SmpenPdCoreLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SMPEN_PD_CORE_B` reader - status of smpen_pd_core_b"]
pub type SmpenPdCoreBR = crate::FieldReader;
#[doc = "Field `SMPEN_PD_CORE_B` writer - status of smpen_pd_core_b"]
pub type SmpenPdCoreBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STANDBYWFIL2_PD_CORE_L` reader - standbywfil2 of pd_core_l status bit"]
pub type Standbywfil2PdCoreLR = crate::BitReader;
#[doc = "Field `STANDBYWFIL2_PD_CORE_L` writer - standbywfil2 of pd_core_l status bit"]
pub type Standbywfil2PdCoreLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STANDBYWFIL2_PD_CORE_B` reader - standbywfil2 of pd_core_b status bit"]
pub type Standbywfil2PdCoreBR = crate::BitReader;
#[doc = "Field `STANDBYWFIL2_PD_CORE_B` writer - standbywfil2 of pd_core_b status bit"]
pub type Standbywfil2PdCoreBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLREMONACK_PD_CORE_L` reader - the status of clremonack_pd_core_l"]
pub type ClremonackPdCoreLR = crate::BitReader;
#[doc = "Field `CLREMONACK_PD_CORE_L` writer - the status of clremonack_pd_core_l"]
pub type ClremonackPdCoreLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLREMONACK_PD_CORE_B` reader - the status of clremonack_pd_core_b"]
pub type ClremonackPdCoreBR = crate::BitReader;
#[doc = "Field `CLREMONACK_PD_CORE_B` writer - the status of clremonack_pd_core_b"]
pub type ClremonackPdCoreBW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - standbywfi of pd_core_l status bit"]
    #[inline(always)]
    pub fn standbywfi_pd_core_l(&self) -> StandbywfiPdCoreLR {
        StandbywfiPdCoreLR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - standbywfe of pd_core_l status bit"]
    #[inline(always)]
    pub fn standbywfe_pd_core_l(&self) -> StandbywfePdCoreLR {
        StandbywfePdCoreLR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - standbywfi of pd_core_b status bit"]
    #[inline(always)]
    pub fn standbywfi_pd_core_b(&self) -> StandbywfiPdCoreBR {
        StandbywfiPdCoreBR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - standbywfe of pd_core_b status bit"]
    #[inline(always)]
    pub fn standbywfe_pd_core_b(&self) -> StandbywfePdCoreBR {
        StandbywfePdCoreBR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19 - status of smpen_pd_core_l"]
    #[inline(always)]
    pub fn smpen_pd_core_l(&self) -> SmpenPdCoreLR {
        SmpenPdCoreLR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - status of smpen_pd_core_b"]
    #[inline(always)]
    pub fn smpen_pd_core_b(&self) -> SmpenPdCoreBR {
        SmpenPdCoreBR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - standbywfil2 of pd_core_l status bit"]
    #[inline(always)]
    pub fn standbywfil2_pd_core_l(&self) -> Standbywfil2PdCoreLR {
        Standbywfil2PdCoreLR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - standbywfil2 of pd_core_b status bit"]
    #[inline(always)]
    pub fn standbywfil2_pd_core_b(&self) -> Standbywfil2PdCoreBR {
        Standbywfil2PdCoreBR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - the status of clremonack_pd_core_l"]
    #[inline(always)]
    pub fn clremonack_pd_core_l(&self) -> ClremonackPdCoreLR {
        ClremonackPdCoreLR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - the status of clremonack_pd_core_b"]
    #[inline(always)]
    pub fn clremonack_pd_core_b(&self) -> ClremonackPdCoreBR {
        ClremonackPdCoreBR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - standbywfi of pd_core_l status bit"]
    #[inline(always)]
    #[must_use]
    pub fn standbywfi_pd_core_l(&mut self) -> StandbywfiPdCoreLW<CpuStatus1Spec> {
        StandbywfiPdCoreLW::new(self, 0)
    }
    #[doc = "Bits 4:7 - standbywfe of pd_core_l status bit"]
    #[inline(always)]
    #[must_use]
    pub fn standbywfe_pd_core_l(&mut self) -> StandbywfePdCoreLW<CpuStatus1Spec> {
        StandbywfePdCoreLW::new(self, 4)
    }
    #[doc = "Bits 8:9 - standbywfi of pd_core_b status bit"]
    #[inline(always)]
    #[must_use]
    pub fn standbywfi_pd_core_b(&mut self) -> StandbywfiPdCoreBW<CpuStatus1Spec> {
        StandbywfiPdCoreBW::new(self, 8)
    }
    #[doc = "Bits 12:13 - standbywfe of pd_core_b status bit"]
    #[inline(always)]
    #[must_use]
    pub fn standbywfe_pd_core_b(&mut self) -> StandbywfePdCoreBW<CpuStatus1Spec> {
        StandbywfePdCoreBW::new(self, 12)
    }
    #[doc = "Bits 16:19 - status of smpen_pd_core_l"]
    #[inline(always)]
    #[must_use]
    pub fn smpen_pd_core_l(&mut self) -> SmpenPdCoreLW<CpuStatus1Spec> {
        SmpenPdCoreLW::new(self, 16)
    }
    #[doc = "Bits 20:21 - status of smpen_pd_core_b"]
    #[inline(always)]
    #[must_use]
    pub fn smpen_pd_core_b(&mut self) -> SmpenPdCoreBW<CpuStatus1Spec> {
        SmpenPdCoreBW::new(self, 20)
    }
    #[doc = "Bit 24 - standbywfil2 of pd_core_l status bit"]
    #[inline(always)]
    #[must_use]
    pub fn standbywfil2_pd_core_l(&mut self) -> Standbywfil2PdCoreLW<CpuStatus1Spec> {
        Standbywfil2PdCoreLW::new(self, 24)
    }
    #[doc = "Bit 25 - standbywfil2 of pd_core_b status bit"]
    #[inline(always)]
    #[must_use]
    pub fn standbywfil2_pd_core_b(&mut self) -> Standbywfil2PdCoreBW<CpuStatus1Spec> {
        Standbywfil2PdCoreBW::new(self, 25)
    }
    #[doc = "Bit 26 - the status of clremonack_pd_core_l"]
    #[inline(always)]
    #[must_use]
    pub fn clremonack_pd_core_l(&mut self) -> ClremonackPdCoreLW<CpuStatus1Spec> {
        ClremonackPdCoreLW::new(self, 26)
    }
    #[doc = "Bit 27 - the status of clremonack_pd_core_b"]
    #[inline(always)]
    #[must_use]
    pub fn clremonack_pd_core_b(&mut self) -> ClremonackPdCoreBW<CpuStatus1Spec> {
        ClremonackPdCoreBW::new(self, 27)
    }
}
#[doc = "cpu status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_status1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_status1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuStatus1Spec;
impl crate::RegisterSpec for CpuStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_status1::R`](R) reader structure"]
impl crate::Readable for CpuStatus1Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_status1::W`](W) writer structure"]
impl crate::Writable for CpuStatus1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU_STATUS1 to value 0"]
impl crate::Resettable for CpuStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
