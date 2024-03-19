#[doc = "Register `DDR_DENALI_CTL_158` reader"]
pub type R = crate::R<DdrDenaliCtl158Spec>;
#[doc = "Register `DDR_DENALI_CTL_158` writer"]
pub type W = crate::W<DdrDenaliCtl158Spec>;
#[doc = "Field `MR_FSP_DATA_VALID_F0_1` reader - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
pub type MrFspDataValidF0_1R = crate::BitReader;
#[doc = "Field `MR_FSP_DATA_VALID_F0_1` writer - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
pub type MrFspDataValidF0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR_FSP_DATA_VALID_F1_1` reader - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
pub type MrFspDataValidF1_1R = crate::BitReader;
#[doc = "Field `MR_FSP_DATA_VALID_F1_1` writer - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
pub type MrFspDataValidF1_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR_FSP_DATA_VALID_F2_1` reader - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
pub type MrFspDataValidF2_1R = crate::BitReader;
#[doc = "Field `MR_FSP_DATA_VALID_F2_1` writer - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
pub type MrFspDataValidF2_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR16_DATA_1` reader - Data to program into memory mode register 16 for chip select 1."]
pub type Mr16Data1R = crate::FieldReader;
#[doc = "Field `MR16_DATA_1` writer - Data to program into memory mode register 16 for chip select 1."]
pub type Mr16Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
    #[inline(always)]
    pub fn mr_fsp_data_valid_f0_1(&self) -> MrFspDataValidF0_1R {
        MrFspDataValidF0_1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
    #[inline(always)]
    pub fn mr_fsp_data_valid_f1_1(&self) -> MrFspDataValidF1_1R {
        MrFspDataValidF1_1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
    #[inline(always)]
    pub fn mr_fsp_data_valid_f2_1(&self) -> MrFspDataValidF2_1R {
        MrFspDataValidF2_1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Data to program into memory mode register 16 for chip select 1."]
    #[inline(always)]
    pub fn mr16_data_1(&self) -> Mr16Data1R {
        Mr16Data1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
    #[inline(always)]
    #[must_use]
    pub fn mr_fsp_data_valid_f0_1(&mut self) -> MrFspDataValidF0_1W<DdrDenaliCtl158Spec> {
        MrFspDataValidF0_1W::new(self, 0)
    }
    #[doc = "Bit 8 - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
    #[inline(always)]
    #[must_use]
    pub fn mr_fsp_data_valid_f1_1(&mut self) -> MrFspDataValidF1_1W<DdrDenaliCtl158Spec> {
        MrFspDataValidF1_1W::new(self, 8)
    }
    #[doc = "Bit 16 - Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter(s). Value of 1 means memory was trained."]
    #[inline(always)]
    #[must_use]
    pub fn mr_fsp_data_valid_f2_1(&mut self) -> MrFspDataValidF2_1W<DdrDenaliCtl158Spec> {
        MrFspDataValidF2_1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Data to program into memory mode register 16 for chip select 1."]
    #[inline(always)]
    #[must_use]
    pub fn mr16_data_1(&mut self) -> Mr16Data1W<DdrDenaliCtl158Spec> {
        Mr16Data1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_158::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_158::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl158Spec;
impl crate::RegisterSpec for DdrDenaliCtl158Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_158::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl158Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_158::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl158Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_158 to value 0"]
impl crate::Resettable for DdrDenaliCtl158Spec {
    const RESET_VALUE: u32 = 0;
}
