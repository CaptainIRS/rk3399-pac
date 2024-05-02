#[doc = "Register `PIPE_RCV_DET_INH` reader"]
pub type R = crate::R<PipeRcvDetInhSpec>;
#[doc = "Register `PIPE_RCV_DET_INH` writer"]
pub type W = crate::W<PipeRcvDetInhSpec>;
#[doc = "Field `FIELD0` reader - Receiver Detect Inhibit Counter Load Value: Counter load value \n\nto delay receiver detection request to PMA until PMA common mode \n\nis within the required range. The timer starts once the PHY signals \n\nready by de-assertion of pipe_phystatus. If receiver detect request \n\nis received while timer has not expired, the PCS will wait until the \n\ntimer expires before signaling the request to the PMA. Load value is \n\nspecified in multiples of 128 ns with a default value of 2 ms. Note: \n\nUnder normal operation the effect of this timer is transparent since \n\nthe USB controller's LTSSM state machine will wait 12 ms in the \n\neSS.Inactive.Quiet state before performing requesting a receiver \n\ndetect operation."]
pub type Field0R = crate::FieldReader<u16>;
#[doc = "Field `FIELD0` writer - Receiver Detect Inhibit Counter Load Value: Counter load value \n\nto delay receiver detection request to PMA until PMA common mode \n\nis within the required range. The timer starts once the PHY signals \n\nready by de-assertion of pipe_phystatus. If receiver detect request \n\nis received while timer has not expired, the PCS will wait until the \n\ntimer expires before signaling the request to the PMA. Load value is \n\nspecified in multiples of 128 ns with a default value of 2 ms. Note: \n\nUnder normal operation the effect of this timer is transparent since \n\nthe USB controller's LTSSM state machine will wait 12 ms in the \n\neSS.Inactive.Quiet state before performing requesting a receiver \n\ndetect operation."]
pub type Field0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Receiver Detect Inhibit Counter Load Value: Counter load value \n\nto delay receiver detection request to PMA until PMA common mode \n\nis within the required range. The timer starts once the PHY signals \n\nready by de-assertion of pipe_phystatus. If receiver detect request \n\nis received while timer has not expired, the PCS will wait until the \n\ntimer expires before signaling the request to the PMA. Load value is \n\nspecified in multiples of 128 ns with a default value of 2 ms. Note: \n\nUnder normal operation the effect of this timer is transparent since \n\nthe USB controller's LTSSM state machine will wait 12 ms in the \n\neSS.Inactive.Quiet state before performing requesting a receiver \n\ndetect operation."]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receiver Detect Inhibit Counter Load Value: Counter load value \n\nto delay receiver detection request to PMA until PMA common mode \n\nis within the required range. The timer starts once the PHY signals \n\nready by de-assertion of pipe_phystatus. If receiver detect request \n\nis received while timer has not expired, the PCS will wait until the \n\ntimer expires before signaling the request to the PMA. Load value is \n\nspecified in multiples of 128 ns with a default value of 2 ms. Note: \n\nUnder normal operation the effect of this timer is transparent since \n\nthe USB controller's LTSSM state machine will wait 12 ms in the \n\neSS.Inactive.Quiet state before performing requesting a receiver \n\ndetect operation."]
    #[inline(always)]
    #[must_use]
    pub fn field0(&mut self) -> Field0W<PipeRcvDetInhSpec> {
        Field0W::new(self, 0)
    }
}
#[doc = "PIPE receiver detect inhibit register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipe_rcv_det_inh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipe_rcv_det_inh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PipeRcvDetInhSpec;
impl crate::RegisterSpec for PipeRcvDetInhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pipe_rcv_det_inh::R`](R) reader structure"]
impl crate::Readable for PipeRcvDetInhSpec {}
#[doc = "`write(|w| ..)` method takes [`pipe_rcv_det_inh::W`](W) writer structure"]
impl crate::Writable for PipeRcvDetInhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PIPE_RCV_DET_INH to value 0x3d09"]
impl crate::Resettable for PipeRcvDetInhSpec {
    const RESET_VALUE: u16 = 0x3d09;
}
