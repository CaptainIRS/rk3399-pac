#[doc = "Register `PIPE_COM_LOCK_CFG2` reader"]
pub type R = crate::R<PipeComLockCfg2Spec>;
#[doc = "Register `PIPE_COM_LOCK_CFG2` writer"]
pub type W = crate::W<PipeComLockCfg2Spec>;
#[doc = "Field `FIELD1` reader - comma lock count: The number of COMMA symbols that needs to \n\nbe seen in the same bit position for the comma state machine to \n\nlock. This field is used while the PCS is transitioning back to the P0 \n\npower state."]
pub type Field1R = crate::FieldReader;
#[doc = "Field `FIELD1` writer - comma lock count: The number of COMMA symbols that needs to \n\nbe seen in the same bit position for the comma state machine to \n\nlock. This field is used while the PCS is transitioning back to the P0 \n\npower state."]
pub type Field1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FIELD0` reader - Reserved"]
pub type Field0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - comma lock count: The number of COMMA symbols that needs to \n\nbe seen in the same bit position for the comma state machine to \n\nlock. This field is used while the PCS is transitioning back to the P0 \n\npower state."]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - comma lock count: The number of COMMA symbols that needs to \n\nbe seen in the same bit position for the comma state machine to \n\nlock. This field is used while the PCS is transitioning back to the P0 \n\npower state."]
    #[inline(always)]
    #[must_use]
    pub fn field1(&mut self) -> Field1W<PipeComLockCfg2Spec> {
        Field1W::new(self, 0)
    }
}
#[doc = "PIPE comma lock configuration2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipe_com_lock_cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipe_com_lock_cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PipeComLockCfg2Spec;
impl crate::RegisterSpec for PipeComLockCfg2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pipe_com_lock_cfg2::R`](R) reader structure"]
impl crate::Readable for PipeComLockCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`pipe_com_lock_cfg2::W`](W) writer structure"]
impl crate::Writable for PipeComLockCfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PIPE_COM_LOCK_CFG2 to value 0x20"]
impl crate::Resettable for PipeComLockCfg2Spec {
    const RESET_VALUE: u16 = 0x20;
}
