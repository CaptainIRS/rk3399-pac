#[doc = "Register `DENALI_CTL_122` reader"]
pub type R = crate::R<DenaliCtl122Spec>;
#[doc = "Register `DENALI_CTL_122` writer"]
pub type W = crate::W<DenaliCtl122Spec>;
#[doc = "Field `MRW_DFS_UPDATE_FRC` reader - Defines the frequency register set to use when doing a software MRW with WRITE_MODEREG bit (26)."]
pub type MrwDfsUpdateFrcR = crate::FieldReader;
#[doc = "Field `MRW_DFS_UPDATE_FRC` writer - Defines the frequency register set to use when doing a software MRW with WRITE_MODEREG bit (26)."]
pub type MrwDfsUpdateFrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TVRCG_ENABLE_F0` reader - JEDEC TVRCG_ENABLE time."]
pub type TvrcgEnableF0R = crate::FieldReader<u16>;
#[doc = "Field `TVRCG_ENABLE_F0` writer - JEDEC TVRCG_ENABLE time."]
pub type TvrcgEnableF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:1 - Defines the frequency register set to use when doing a software MRW with WRITE_MODEREG bit (26)."]
    #[inline(always)]
    pub fn mrw_dfs_update_frc(&self) -> MrwDfsUpdateFrcR {
        MrwDfsUpdateFrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:25 - JEDEC TVRCG_ENABLE time."]
    #[inline(always)]
    pub fn tvrcg_enable_f0(&self) -> TvrcgEnableF0R {
        TvrcgEnableF0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Defines the frequency register set to use when doing a software MRW with WRITE_MODEREG bit (26)."]
    #[inline(always)]
    #[must_use]
    pub fn mrw_dfs_update_frc(&mut self) -> MrwDfsUpdateFrcW<DenaliCtl122Spec> {
        MrwDfsUpdateFrcW::new(self, 0)
    }
    #[doc = "Bits 16:25 - JEDEC TVRCG_ENABLE time."]
    #[inline(always)]
    #[must_use]
    pub fn tvrcg_enable_f0(&mut self) -> TvrcgEnableF0W<DenaliCtl122Spec> {
        TvrcgEnableF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_122::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_122::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl122Spec;
impl crate::RegisterSpec for DenaliCtl122Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_122::R`](R) reader structure"]
impl crate::Readable for DenaliCtl122Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_122::W`](W) writer structure"]
impl crate::Writable for DenaliCtl122Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_122 to value 0"]
impl crate::Resettable for DenaliCtl122Spec {
    const RESET_VALUE: u32 = 0;
}
