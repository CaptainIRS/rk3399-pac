#[doc = "Register `DENALI_CTL_144` reader"]
pub type R = crate::R<DenaliCtl144Spec>;
#[doc = "Register `DENALI_CTL_144` writer"]
pub type W = crate::W<DenaliCtl144Spec>;
#[doc = "Field `MR_FSP_DATA_VALID_F0_0` reader - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
pub type MrFspDataValidF0_0R = crate::BitReader;
#[doc = "Field `MR_FSP_DATA_VALID_F0_0` writer - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
pub type MrFspDataValidF0_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR_FSP_DATA_VALID_F1_0` reader - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
pub type MrFspDataValidF1_0R = crate::BitReader;
#[doc = "Field `MR_FSP_DATA_VALID_F1_0` writer - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
pub type MrFspDataValidF1_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR_FSP_DATA_VALID_F2_0` reader - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
pub type MrFspDataValidF2_0R = crate::BitReader;
#[doc = "Field `MR_FSP_DATA_VALID_F2_0` writer - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
pub type MrFspDataValidF2_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR16_DATA_0` reader - Data to program into memory mode register 16 for chip select 0."]
pub type Mr16Data0R = crate::FieldReader;
#[doc = "Field `MR16_DATA_0` writer - Data to program into memory mode register 16 for chip select 0."]
pub type Mr16Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
    #[inline(always)]
    pub fn mr_fsp_data_valid_f0_0(&self) -> MrFspDataValidF0_0R {
        MrFspDataValidF0_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
    #[inline(always)]
    pub fn mr_fsp_data_valid_f1_0(&self) -> MrFspDataValidF1_0R {
        MrFspDataValidF1_0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
    #[inline(always)]
    pub fn mr_fsp_data_valid_f2_0(&self) -> MrFspDataValidF2_0R {
        MrFspDataValidF2_0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Data to program into memory mode register 16 for chip select 0."]
    #[inline(always)]
    pub fn mr16_data_0(&self) -> Mr16Data0R {
        Mr16Data0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
    #[inline(always)]
    #[must_use]
    pub fn mr_fsp_data_valid_f0_0(&mut self) -> MrFspDataValidF0_0W<DenaliCtl144Spec> {
        MrFspDataValidF0_0W::new(self, 0)
    }
    #[doc = "Bit 8 - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
    #[inline(always)]
    #[must_use]
    pub fn mr_fsp_data_valid_f1_0(&mut self) -> MrFspDataValidF1_0W<DenaliCtl144Spec> {
        MrFspDataValidF1_0W::new(self, 8)
    }
    #[doc = "Bit 16 - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
    #[inline(always)]
    #[must_use]
    pub fn mr_fsp_data_valid_f2_0(&mut self) -> MrFspDataValidF2_0W<DenaliCtl144Spec> {
        MrFspDataValidF2_0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Data to program into memory mode register 16 for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn mr16_data_0(&mut self) -> Mr16Data0W<DenaliCtl144Spec> {
        Mr16Data0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_144::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_144::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl144Spec;
impl crate::RegisterSpec for DenaliCtl144Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_144::R`](R) reader structure"]
impl crate::Readable for DenaliCtl144Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_144::W`](W) writer structure"]
impl crate::Writable for DenaliCtl144Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_144 to value 0"]
impl crate::Resettable for DenaliCtl144Spec {
    const RESET_VALUE: u32 = 0;
}
